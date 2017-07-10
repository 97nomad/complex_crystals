use std::sync::{Arc, Mutex};
use server::GameEngine;
use server::events::*;
use rustc_serialize::json;
use data_types::*;

pub fn world_size(mutex: &Arc<Mutex<GameEngine>>) -> Option<String> {
    let engine = mutex.lock().unwrap();
    let response = WorldSizeResponse {
        width: engine.world_size_x,
        height: engine.world_size_y,
    };
    Some(json::encode(&response).unwrap())
}

pub fn info(mutex: &Arc<Mutex<GameEngine>>) -> Option<String> {
    let engine = mutex.lock().unwrap();
    Some(json::encode(&engine.info).unwrap())
}

pub fn objects(mutex: &Arc<Mutex<GameEngine>>) -> Option<String> {
    let engine = mutex.lock().unwrap();
    let objects: Vec<ObjectResponse> = engine
        .objects
        .iter()
        .map(|x| {
            let (_, obj) = x;
            ObjectResponse {
                name: obj.name.clone(),
                owner: obj.owner.clone(),
                x: obj.x,
                y: obj.y,
                otype: obj.otype.clone(),
            }
        })
        .collect();
    Some(json::encode(&objects).unwrap())
}

pub fn object_info(mutex: &Arc<Mutex<GameEngine>>,
                   raw_json: String,
                   owner: String)
                   -> Option<String> {
    println!("{:?}", &raw_json);
    let name: NameResponse = json::decode(&raw_json).unwrap();

    let mut engine = mutex.lock().unwrap();
    if let Some(object) = engine.get_object(&name.name, Some(&owner)) {
        Some(json::encode(object).unwrap())
    } else {
        None
    }
}

pub fn move_object(mutex: &Arc<Mutex<GameEngine>>, input: String, owner: String) -> bool {
    let mut engine = mutex.lock().unwrap();
    match json::decode(&input) {
        Err(e) => {
            println!("Json parsing error: {:?}", e);
            return false;
        }
        Ok(data) => {
            let mvr: MoveObjectRequest = data;

            println!("Передвижение объекта {} -- x: {} y: {}",
                     mvr.name,
                     mvr.x,
                     mvr.y);

            engine.add_event(Event::MoveRequest(NetworkMoveEvent {
                                                    name: mvr.name,
                                                    owner: owner,
                                                    dest_x: mvr.x,
                                                    dest_y: mvr.y,
                                                }));
        }
    }
    true
}

pub fn radar(mutex: &Arc<Mutex<GameEngine>>, owner: String) -> Option<String> {
    let engine = mutex.lock().unwrap();
    let scan_result = engine.radar_scan(&owner, true);
    Some(json::encode(&scan_result).unwrap())
}

pub fn weapon_fire(mutex: &Arc<Mutex<GameEngine>>, request: String, owner: String) -> bool {
    let mut engine = mutex.lock().unwrap();
    match json::decode(&request) {
        Err(e) => {
            println!("Json parsing error: {:?}", e);
            return false;
        }
        Ok(data) => {
            let wfr: WeaponFireRequest = data;
            println!("Огонь объекта {} -- x: {} y: {}",
                     wfr.name,
                     wfr.x,
                     wfr.y);

            engine.add_event(Event::FireRequest(NetworkFireEvent {
                                                    name: wfr.name,
                                                    owner: owner,
                                                    dest_x: wfr.x,
                                                    dest_y: wfr.y,
                                                }));
        }
    }
    true
}

pub fn build(mutex: &Arc<Mutex<GameEngine>>, request: String, owner: String) -> bool {
    let mut engine = mutex.lock().unwrap();
    match json::decode(&request) {
        Err(e) => {
            println!("Json parsing error: {:?}", e);
            return false;
        }
        Ok(data) => {
            let req: BuildRequest = data;

            println!("Постройка объекта {} при помощи {}",
                     req.oname,
                     req.name);

            engine.add_event(Event::BuildRequest(NetworkBuildEvent {
                                                     name: req.name,
                                                     owner: owner,
                                                     b_type: req.otype,
                                                     b_name: req.oname,
                                                 }));

        }
    }
    true
}
