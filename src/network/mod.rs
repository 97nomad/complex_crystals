use rustc_serialize::json;
use hyper::{Client, Url};
use hyper::header::{Authorization, Basic, Headers};
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::collections::HashMap;
use std::mem::replace;

use data_types::{SampleObject, ObjectResponse, ServerInfo, WorldSize, ObjectInfoRequest};

const OBJECTS_UPDATE_ADDR: &'static str = "/objects";
const OBJECTINFO_ADDR: &'static str = "/object_info";
const SERVERINFO_UPDATE_ADDR: &'static str = "/info";
const WORLDSIZE_UPDATE_ADDR: &'static str = "/world_size";
const USERNAME: &'static str = "admin";

pub struct Network {
    pub addr: String,
    pub df_select_object: Arc<Mutex<bool>>,
    pub select_object: Arc<Mutex<SampleObject>>,
    pub objects: Arc<Mutex<HashMap<String, ObjectResponse>>>,
    pub server_info: Arc<Mutex<ServerInfo>>,
    pub world_size: Arc<Mutex<WorldSize>>,
}

impl Network {
    pub fn new(addr: String) -> Self {
        Network {
            addr: addr,
            df_select_object: Arc::new(Mutex::new(true)),
            select_object: Arc::new(Mutex::new(SampleObject::new_empty())),
            objects: Arc::new(Mutex::new(HashMap::new())),
            server_info: Arc::new(Mutex::new(ServerInfo {
                                                 name: "ServerName".to_owned(),
                                                 status: "SomeStatus".to_owned(),
                                                 tps: 0,
                                             })),
            world_size: Arc::new(Mutex::new(WorldSize {
                                                width: 0.0,
                                                height: 0.0,
                                            })),
        }
    }

    pub fn check_connection(&self) -> JoinHandle<ServerInfo> {
        let addr = Url::parse(&format!("http://{}{}", self.addr, SERVERINFO_UPDATE_ADDR)).unwrap();
        thread::spawn(move || NetworkRequest::check_server_info(addr))
    }

    pub fn update_select_object(&mut self, name: String) {
        let addr = Url::parse(&format!("http://{}{}", self.addr, OBJECTINFO_ADDR)).unwrap();
        let select_object = self.select_object.clone();
        let df_select_object = self.df_select_object.clone();
        println!("Spawning thread");
        thread::spawn(move || {
                          NetworkRequest::select_object(name, select_object, df_select_object, addr)
                      });
    }

    pub fn update_objects(&mut self) {
        let addr = Url::parse(&format!("http://{}{}", self.addr, OBJECTS_UPDATE_ADDR)).unwrap();
        let objects = self.objects.clone();
        thread::spawn(move || NetworkRequest::update_objects(objects, addr));
    }
    pub fn update_info(&mut self) {
        let addr = Url::parse(&format!("http://{}{}", self.addr, SERVERINFO_UPDATE_ADDR)).unwrap();
        let server_info = self.server_info.clone();
        thread::spawn(move || NetworkRequest::update_server_info(server_info, addr));
    }
    pub fn update_world_size(&mut self) {
        let addr = Url::parse(&format!("http://{}{}", self.addr, WORLDSIZE_UPDATE_ADDR)).unwrap();
        let world_size = self.world_size.clone();
        thread::spawn(move || NetworkRequest::update_world_size(world_size, addr));
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

    fn select_object(name: String,
                     object: Arc<Mutex<SampleObject>>,
                     df: Arc<Mutex<bool>>,
                     addr: Url) {
        println!("{:?}",
                 json::encode(&ObjectInfoRequest { name: name.clone() }).unwrap());
        let data = NetworkRequest::request(addr,
                                           Some(json::encode(&ObjectInfoRequest { name: name })
                                                    .unwrap()));
        let new_object: SampleObject = json::decode(&data).unwrap();
        let mut object = object.lock().unwrap();
        object.replace_object(new_object);
        *df.lock().unwrap() = false;
        println!("Object selected");
    }

    fn update_objects(objects: Arc<Mutex<HashMap<String, ObjectResponse>>>, addr: Url) {
        let data = NetworkRequest::request(addr, None);

        let mut parsed_objects: Vec<ObjectResponse> = match json::decode(&data) {
            Err(e) => {
                println!("Json parsing error: {:?}", e);
                vec![]
            }
            Ok(data) => data,
        };
        let mut objects = objects.lock().unwrap();
        objects.clear();
        for object in parsed_objects {
            objects.insert(object.name.clone(), object);
        }
    }

    fn update_server_info(server_info: Arc<Mutex<ServerInfo>>, addr: Url) {
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
        let mut serverinfo = server_info.lock().unwrap();
        serverinfo.replace(parsed_info);
    }

    fn check_server_info(addr: Url) -> ServerInfo {
        let data = NetworkRequest::request(addr, None);
        match json::decode(&data) {
            Err(e) => panic!("Server not found"),
            Ok(data) => data,
        }
    }

    fn update_world_size(world_size: Arc<Mutex<WorldSize>>, addr: Url) {
        let data = NetworkRequest::request(addr, None);
        let parsed_info: WorldSize = match json::decode(&data) {
            Err(e) => {
                println!("Json parsing error: {:?}", e);
                WorldSize {
                    width: 0.0,
                    height: 0.0,
                }
            }
            Ok(data) => data,
        };
        let mut world_size = world_size.lock().unwrap();
        world_size.replace(parsed_info);
    }
}
