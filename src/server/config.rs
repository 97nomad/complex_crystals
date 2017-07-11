use rustc_serialize::json;
use std::fs::File;
use std::io::Read;

#[derive(RustcDecodable)]
pub struct GameConfig {
    pub servername: String,
}

impl GameConfig {
    pub fn new(path: &str) -> Self {
        let mut file = match File::open(path) {
            Ok(data) => data,
            Err(e) => panic!("Game config file open error: {:?}", e),
        };
        let mut string = String::new();
        file.read_to_string(&mut string).unwrap();

        match json::decode(&string) {
            Err(e) => {
                panic!("Json parsing error: {:?}", e);
            }
            Ok(data) => data,
        }
    }
}
