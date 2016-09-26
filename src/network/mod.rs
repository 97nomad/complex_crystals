pub mod sampleobject;

use rustc_serialize::json;
use hyper::{Client, Url};
use hyper::header::{Authorization, Basic, Headers};
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;

use self::sampleobject::SampleObject;

pub struct Network {
    pub objects: Arc<Mutex<Vec<SampleObject>>>,
}

impl Network {
    pub fn new() -> Self {
        Network { objects: Arc::new(Mutex::new(vec![])) }
    }

    pub fn update(&mut self, addr: &str) {
        let objects = self.objects.clone();
        let addr = Url::parse(addr).unwrap();
        thread::spawn(move || NetworkRequest::update_objects(objects, addr));
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
        let mut response = client.get(addr)
            .headers(headers)
            .send()
            .unwrap();
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
}