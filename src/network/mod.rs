pub mod sampleobject;

use rustc_serialize::json;
use hyper::{Client, Url};
use hyper::header::{Authorization, Basic, Headers};
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;

use self::sampleobject::{SampleObject, ServerInfo};

pub struct Network {
    pub objects: Arc<Mutex<Vec<SampleObject>>>,
    pub server_info: Arc<Mutex<ServerInfo>>,
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
            Err(e) => {
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
            Err(e) => {
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
}