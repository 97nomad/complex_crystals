pub mod sampleobject;

use rustc_serialize::json;
use hyper::{Client, Url};
use hyper::header::{Authorization, Basic, Headers};
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;

use self::sampleobject::{SampleObject, ServerInfo, WorldSize};

pub struct Network {
    pub objects: Arc<Mutex<Vec<SampleObject>>>,
    pub server_info: Arc<Mutex<ServerInfo>>,
    pub world_size: Arc<Mutex<WorldSize>>,
}

impl Network {
    pub fn new() -> Self {
        Network {
            objects: Arc::new(Mutex::new(vec![])),
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

    pub fn update(&mut self, addr: &str) {
        let objects = self.objects.clone();
        let addr = match Url::parse(addr) {
            Ok(addr) => addr,
            Err(e) => {
                panic!("{:?}", e);
            }
        };
        thread::spawn(move || NetworkRequest::update_objects(objects, addr));
    }
    pub fn update_info(&mut self, addr: &str) {
        let info = self.server_info.clone();
        let addr = match Url::parse(addr) {
            Ok(addr) => addr,
            Err(e) => {
                panic!("{:?}", e);
            }
        };
        thread::spawn(move || NetworkRequest::update_server_info(info, addr));
    }
    pub fn update_world_size(&mut self, addr: &str) {
        let worldsize = self.world_size.clone();
        let addr = match Url::parse(addr) {
            Ok(addr) => addr,
            Err(e) => {
                panic!("{:?}", e);
            }
        };
        thread::spawn(move || NetworkRequest::update_world_size(worldsize, addr));
    }
}

struct NetworkRequest {}

impl NetworkRequest {
    fn update_objects(objects: Arc<Mutex<Vec<SampleObject>>>, addr: Url) {
        let client = Client::new();

        let mut headers = Headers::new();
        headers.set(Authorization(Basic {
            username: "admin".to_owned(),
            password: None,
        }));
        let mut response = match client.get(addr)
            .headers(headers)
            .send() {
            Ok(data) => data,
            Err(_) => {
                panic!("Сервер не ответил на запрос");
            }
        };
        let mut response_string = String::new();
        response.read_to_string(&mut response_string).unwrap();
        let mut parsed_objects: Vec<SampleObject> = match json::decode(&response_string) {
            Err(e) => {
                println!("Json parsing error: {:?}", e);
                vec![]
            }
            Ok(data) => data,
        };
        let mut objects = objects.lock().unwrap();
        objects.drain(..);
        objects.append(&mut parsed_objects);
    }

    fn update_server_info(server_info: Arc<Mutex<ServerInfo>>, addr: Url) {
        let client = Client::new();

        let mut headers = Headers::new();
        headers.set(Authorization(Basic {
            username: "admin".to_owned(),
            password: None,
        }));
        let mut response = match client.get(addr)
            .headers(headers)
            .send() {
            Ok(data) => data,
            Err(_) => {
                panic!("Сервер не ответил на запрос");
            }
        };
        let mut response_string = String::new();
        response.read_to_string(&mut response_string).unwrap();
        let parsed_info: ServerInfo = match json::decode(&response_string) {
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
        let client = Client::new();

        let mut headers = Headers::new();
        headers.set(Authorization(Basic {
            username: "admin".to_owned(),
            password: None,
        }));
        let mut response = match client.get(addr)
            .headers(headers)
            .send() {
            Ok(data) => data,
            Err(_) => {
                panic!("Сервер не ответил на запрос");
            }
        };
        let mut response_string = String::new();
        response.read_to_string(&mut response_string).unwrap();
        let parsed_info: WorldSize = match json::decode(&response_string) {
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