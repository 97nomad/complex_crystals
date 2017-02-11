pub mod sampleobject;

use rustc_serialize::json;
use hyper::{Client, Url};
use hyper::header::{Authorization, Basic, Headers};
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::HashMap;

use self::sampleobject::{SampleObject, ObjectResponse, ServerInfo, WorldSize};

const OBJECTS_UPDATE_ARRD: &'static str = "http://localhost:3000/objects";
const SERVERINFO_UPDATE_ADDR: &'static str = "http://localhost:3000/info";
const WORLDSIZE_UPDATE_ADDR: &'static str = "http://localhost:3000/world_size";
const USERNAME: &'static str = "admin";

pub struct Network {
    pub df_select_object: bool,
    pub select_object: Arc<Mutex<Option<SampleObject>>>,
    pub objects: Arc<Mutex<HashMap<String, ObjectResponse>>>,
    pub server_info: Arc<Mutex<ServerInfo>>,
    pub world_size: Arc<Mutex<WorldSize>>,
}

impl Network {
    pub fn new() -> Self {
        Network {
            df_select_object: true,
            select_object: Arc::new(Mutex::new(None)),
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

    pub fn update_objects(&mut self) {
        let addr = Url::parse(OBJECTS_UPDATE_ARRD).unwrap();
        let objects = self.objects.clone();
        thread::spawn(move || NetworkRequest::update_objects(objects, addr));
    }
    pub fn update_info(&mut self) {
        let addr = Url::parse(SERVERINFO_UPDATE_ADDR).unwrap();
        let server_info = self.server_info.clone();
        thread::spawn(move || NetworkRequest::update_server_info(server_info, addr));
    }
    pub fn update_world_size(&mut self) {
        let addr = Url::parse(WORLDSIZE_UPDATE_ADDR).unwrap();
        let world_size = self.world_size.clone();
        thread::spawn(move || NetworkRequest::update_world_size(world_size, addr));
    }
}

struct NetworkRequest {}

impl NetworkRequest {
    fn request(addr: Url) -> String {
        let client = Client::new();

        let mut headers = Headers::new();
        headers.set(Authorization(Basic {
            username: USERNAME.to_owned(),
            password: None,
        }));

        let mut response = match client.get(addr).headers(headers).send() {
            Ok(data) => data,
            Err(_) => panic!("Сервер не ответил на запрос"),
        };
        let mut result_string = String::new();
        response.read_to_string(&mut result_string).unwrap();
        result_string
    }

    fn update_objects(objects: Arc<Mutex<HashMap<String, ObjectResponse>>>, addr: Url) {
        let data = NetworkRequest::request(addr);

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
        let data = NetworkRequest::request(addr);
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

    fn update_world_size(world_size: Arc<Mutex<WorldSize>>, addr: Url) {
        let data = NetworkRequest::request(addr);
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