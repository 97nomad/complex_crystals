use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread::spawn;

use network::ServerConnection;
use server::GameEngine;
use server::network;
use data_types::{SampleObject, ObjectResponse, ServerInfo};
use level_generator::generate;

pub struct ServerManager {
    engine_timer: f64,
    tps_timer: f64,

    tps: u16,
    selected_object: Option<String>,
    engine: Arc<Mutex<GameEngine>>,
}

impl ServerManager {
    pub fn new(width: f64, height: f64, players: Vec<String>) -> Self {
        let engine = Arc::new(Mutex::new(GameEngine::new(width, height)));

        generate(engine.clone(), width, height, players);
        let cloned_engine = engine.clone();
        spawn(move || network::start(cloned_engine));

        ServerManager {
            engine_timer: 0.0,
            tps_timer: 0.0,
            tps: 0,
            selected_object: None,
            engine: engine,
        }
    }
}

impl ServerConnection for ServerManager {
    fn update(&mut self, elapsed: f64) {
        self.engine_timer += elapsed;
        self.tps_timer += elapsed;
        // 60 TPS
        if self.engine_timer >= 1.0 / 60.0 {
            self.engine_timer = 0.0;
            self.tps += 1;

            let mut engine = self.engine.lock().unwrap();

            if self.tps_timer >= 1.0 {
                engine.update_tps(self.tps);
                self.tps = 0;
            }

            engine.game_loop(elapsed);

        }
    }
    fn check_connection(&self) -> Option<ServerInfo> {
        Some(self.engine.lock().unwrap().get_server_info())
    }
    fn get_objects(&self) -> HashMap<String, ObjectResponse> {
        self.engine.lock().unwrap().get_objects()
    }
    fn select_object(&mut self, name: String) {
        self.selected_object = Some(name);
    }
    fn get_selected_object(&self) -> Option<SampleObject> {
        if let Some(ref name) = self.selected_object {
            if let Some(obj) = self.engine.lock().unwrap().get_object(&name, None) {
                Some(obj.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
    fn get_server_info(&self) -> ServerInfo {
        ServerInfo {
            name: "ServerName".to_owned(),
            status: "SomeStatus".to_owned(),
            tps: 0,
        }
    }
}
