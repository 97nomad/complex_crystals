use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use hyper::{Client, Url};
use hyper::header::{Authorization, Basic, Headers};
use std::io::Read;
use std::thread;
use std::thread::JoinHandle;
use rustc_serialize::json;
use data_types::{SampleObject, ObjectResponse, ServerInfo, ObjectInfoRequest};

use network::ServerConnection;

const OBJECTS_UPDATE_ADDR: &'static str = "/objects";
const OBJECTINFO_ADDR: &'static str = "/object_info";
const SERVERINFO_UPDATE_ADDR: &'static str = "/info";
const USERNAME: &'static str = "admin";

pub struct ServerClient {
    addr: String,
    update_timer: f64,
    thread_check_timer: f64,

    df_objects: Arc<Mutex<bool>>,
    df_selected_object: Arc<Mutex<bool>>,
    df_server_info: Arc<Mutex<bool>>,

    jh_objects: Option<JoinHandle<HashMap<String, ObjectResponse>>>,
    jh_selected_object: Option<JoinHandle<SampleObject>>,
    jh_server_info: Option<JoinHandle<ServerInfo>>,

    selected_object: Option<SampleObject>,
    objects: HashMap<String, ObjectResponse>,
    server_info: ServerInfo,
}

impl ServerClient {
    pub fn new(addr: String) -> Self {
        ServerClient {
            addr: addr,
            update_timer: 3.0,
            thread_check_timer: 0.0,

            df_objects: Arc::new(Mutex::new(false)),
            df_selected_object: Arc::new(Mutex::new(false)),
            df_server_info: Arc::new(Mutex::new(false)),

            jh_objects: None,
            jh_selected_object: None,
            jh_server_info: None,

            selected_object: None,
            objects: HashMap::new(),
            server_info: ServerInfo {
                name: "ServerName".to_owned(),
                status: "SomeStatus".to_owned(),
                tps: 0,
            },
        }
    }
}

impl ServerConnection for ServerClient {
    fn update(&mut self, elapsed: f64) {
        self.update_timer += elapsed;
        self.thread_check_timer += elapsed;
        if self.update_timer >= 3.0 {
            self.update_timer = 0.0;
            // Update objects
            if self.jh_objects.is_none() {
                let addr = Url::parse(&format!("http://{}{}", self.addr, OBJECTS_UPDATE_ADDR))
                    .unwrap();
                let flag_mutex = self.df_objects.clone();
                self.jh_objects =
                    Some(thread::spawn(move || NetworkRequest::update_objects(flag_mutex, addr)));
            }

            // Update server info
            if self.jh_server_info.is_none() {
                let addr = Url::parse(&format!("http://{}{}", self.addr, SERVERINFO_UPDATE_ADDR))
                    .unwrap();
                let flag_mutex = self.df_objects.clone();
                self.jh_server_info = Some(thread::spawn(move || {
                    NetworkRequest::update_server_info(flag_mutex, addr)
                }));
            }
        }

        if self.thread_check_timer >= 0.1 {
            self.thread_check_timer = 0.0;
            // When threads is end
            let mut df_objects = self.df_objects.lock().unwrap();
            if *df_objects == true {
                self.objects = self.jh_objects.take().unwrap().join().unwrap();
                *df_objects = false;
            }

            let mut df_server_info = self.df_server_info.lock().unwrap();
            if *df_server_info == true {
                self.server_info = self.jh_server_info.take().unwrap().join().unwrap();
                *df_server_info = false;
            }

            let mut df_selected_object = self.df_selected_object.lock().unwrap();
            if *df_selected_object == true {
                self.selected_object =
                    Some(self.jh_selected_object.take().unwrap().join().unwrap());
                *df_selected_object = false;
            }
        }
    }
    fn check_connection(&self) -> Option<ServerInfo> {
        let addr = Url::parse(&format!("http://{}{}", self.addr, SERVERINFO_UPDATE_ADDR)).unwrap();
        thread::spawn(move || NetworkRequest::check_server_info(addr))
            .join()
            .ok()
    }

    fn get_objects(&self) -> HashMap<String, ObjectResponse> {
        self.objects.clone()
    }

    fn select_object(&mut self, name: String) {
        let addr = Url::parse(&format!("http://{}{}", self.addr, OBJECTINFO_ADDR)).unwrap();
        let flag_mutex = self.df_selected_object.clone();
        self.jh_selected_object =
            Some(thread::spawn(move || NetworkRequest::select_object(name, flag_mutex, addr)))
    }

    fn get_selected_object(&self) -> Option<SampleObject> {
        self.selected_object.clone()
    }
    fn get_server_info(&self) -> ServerInfo {
        self.server_info.clone()
    }
}

struct NetworkRequest {}

impl NetworkRequest {
    fn request(addr: Url, payload: Option<String>) -> String {
        let client = Client::new();

        let mut headers = Headers::new();
        headers.set(Authorization(Basic {
                                      username: USERNAME.to_owned(),
                                      password: None,
                                  }));

        let payload = payload.unwrap_or("".to_owned());
        let mut response = match client.get(addr).headers(headers).body(&payload).send() {
            Ok(data) => data,
            Err(_) => panic!("Server not responding"),
        };
        let mut result_string = String::new();
        response.read_to_string(&mut result_string).unwrap();
        result_string
    }

    fn select_object(name: String, df: Arc<Mutex<bool>>, addr: Url) -> SampleObject {
        println!("{:?}",
                 json::encode(&ObjectInfoRequest { name: name.clone() }).unwrap());
        let data = NetworkRequest::request(addr,
                                           Some(json::encode(&ObjectInfoRequest { name: name })
                                                    .unwrap()));
        *df.lock().unwrap() = true;
        println!("Object selected");
        json::decode::<SampleObject>(&data).unwrap()
    }

    fn update_objects(df: Arc<Mutex<bool>>, addr: Url) -> HashMap<String, ObjectResponse> {
        let data = NetworkRequest::request(addr, None);

        let parsed_objects: Vec<ObjectResponse> = match json::decode(&data) {
            Err(e) => {
                println!("Json parsing error: {:?}", e);
                vec![]
            }
            Ok(data) => data,
        };
        let mut objects = HashMap::new();
        for object in parsed_objects {
            objects.insert(object.name.clone(), object);
        }
        *df.lock().unwrap() = true;
        objects
    }

    fn update_server_info(df: Arc<Mutex<bool>>, addr: Url) -> ServerInfo {
        let data = NetworkRequest::request(addr, None);
        let parsed_info: ServerInfo = match json::decode(&data) {
            Err(e) => {
                println!("Json parsing error: {:?}", e);
                ServerInfo {
                    name: "ErrorName".to_owned(),
                    status: "ErrorStatus".to_owned(),
                    tps: 0,
                }
            }
            Ok(data) => data,
        };
        *df.lock().unwrap() = true;
        parsed_info
    }

    fn check_server_info(addr: Url) -> ServerInfo {
        let data = NetworkRequest::request(addr, None);
        match json::decode(&data) {
            Err(_) => panic!("Server not found"),
            Ok(data) => data,
        }
    }
}
