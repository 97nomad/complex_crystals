pub mod server_client;

pub use self::server_client::ServerClient;

use std::collections::HashMap;
use data_types::{SampleObject, ObjectResponse, ServerInfo, WorldSize, ObjectInfoRequest};

pub trait ServerConnection {
    fn update(&mut self, elapsed: f64);
    fn check_connection(&self) -> Option<ServerInfo>;

    fn get_objects(&self) -> HashMap<String, ObjectResponse>;
    fn select_object(&mut self, name: String);
    fn get_selected_object(&self) -> Option<SampleObject>;
    fn get_server_info(&self) -> ServerInfo;
}
