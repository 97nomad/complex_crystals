pub mod server_client;
pub mod server_manager;

pub use self::server_client::ServerClient;
pub use self::server_manager::ServerManager;

use std::collections::HashMap;
use data_types::{SampleObject, ObjectResponse, ServerInfo};

pub trait ServerConnection {
    fn update(&mut self, elapsed: f64);
    fn check_connection(&self) -> Option<ServerInfo>;

    fn get_objects(&self) -> HashMap<String, ObjectResponse>;
    fn select_object(&mut self, name: String);
    fn get_selected_object(&self) -> Option<SampleObject>;
    fn get_server_info(&self) -> ServerInfo;
}
