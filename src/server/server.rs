use data_types::*;
use server::events::*;
use server::config::GameConfig;
use std::collections::{HashMap, VecDeque};

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct ServerInfo {
    name: String,
    status: String,
    tps: u16,
}

pub struct GameEngine {
    pub info: ServerInfo,
    pub objects: HashMap<String, SampleObject>,
    pub world_size_x: f64,
    pub world_size_y: f64,
    pub events: VecDeque<Event>,
    pub config: GameConfig,
}

impl GameEngine {
    pub fn new(width: f64, height: f64) -> Self {
        let config = GameConfig::new("config/engine.json");
        GameEngine {
            objects: HashMap::new(),
            info: ServerInfo {
                name: config.servername.clone(),
                status: "Ok".to_string(),
                tps: 0u16,
            },
            world_size_x: width,
            world_size_y: height,
            events: VecDeque::new(),
            config: config,
        }
    }
    pub fn update_tps(&mut self, tps: u16) {
        self.info.tps = tps;
    }

    pub fn add_object(&mut self,
                      object_name: String,
                      coord_x: f64,
                      coord_y: f64,
                      otype: ObjectType,
                      owner: String) {
        self.objects
            .insert(object_name.clone(),
                    SampleObject::new(owner, object_name, otype, coord_x, coord_y));
    }

    pub fn get_object_with_owner(&self, name: String, owner: String) -> Option<SampleObject> {
        match self.objects.get(&name) {
            Some(obj) => {
                if obj.owner != owner {
                    return None;
                } else {
                    return Some(obj.clone());
                }
            }
            None => return None, 
        };
    }

    pub fn check_object_owner(&self, object: &SampleObject, owner: Option<&String>) -> bool {
        if let Some(owner) = owner {
            object.owner.eq(owner)
        } else {
            true
        }
    }

    pub fn check_object_exsists(&self, name: &String, owner: Option<&String>) -> bool {
        if let Some(object) = self.objects.get(name) {
            if let Some(owner) = owner {
                object.owner.eq(owner)
            } else {
                true
            }
        } else {
            false
        }
    }

    pub fn get_object_mut(&mut self,
                          name: &String,
                          owner: Option<&String>)
                          -> Option<&mut SampleObject> {
        if let Some(object) = self.objects.get_mut(name) {
            if let Some(owner) = owner {
                if object.owner.eq(owner) {
                    Some(object)
                } else {
                    None
                }
            } else {
                Some(object)
            }
        } else {
            None
        }
    }

    pub fn get_object(&mut self, name: &String, owner: Option<&String>) -> Option<&SampleObject> {
        if let Some(object) = self.objects.get(name) {
            if let Some(owner) = owner {
                if object.owner.eq(owner) {
                    Some(object)
                } else {
                    None
                }
            } else {
                Some(object)
            }
        } else {
            None
        }
    }

    pub fn game_loop(&mut self, elapsed: f64) {
        self.event(elapsed);
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push_front(event);
    }

    pub fn radar_scan(&self, owner: &String, only_enemies: bool) -> Vec<(f64, f64)> {
        let mut radars = vec![];
        for (_, obj) in self.objects.iter() {
            if obj.check_owner(Some(owner)) {
                radars.push(((obj.x, obj.y), obj.radar_radius));
            }
        }
        let mut result_vec = vec![];
        for (_, obj) in self.objects.iter() {
            if only_enemies && !obj.check_owner(Some(owner)) {
                result_vec.push((obj.x, obj.y));
            }
        }
        result_vec
    }

    fn event(&mut self, elapsed: f64) {
        let event = match self.events.pop_back() {
            Some(e) => e,
            None => return,
        };
        let return_event: Option<Event> = match event {
            Event::MoveRequest(m_e) => {
                if self.check_object_exsists(&m_e.name, Some(&m_e.owner)) {
                    Some(Event::Move(MoveEvent {
                                         name: m_e.name,
                                         dest_x: m_e.dest_x,
                                         dest_y: m_e.dest_y,
                                     }))
                } else {
                    None
                }
            }

            Event::FireRequest(f_e) => {
                if let Some(object) = self.get_object_mut(&f_e.name, Some(&f_e.owner)) {
                    if object.cargo_remove(1.0) {
                        Some(Event::Damage(DamageEvent {
                                               x: f_e.dest_x,
                                               y: f_e.dest_y,
                                               size: object.weapon_radius,
                                               d_type: object.weapon_type.clone(),
                                               damage: 10.0,
                                           }))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Event::BuildRequest(b_e) => {
                if self.check_object_exsists(&b_e.name, Some(&b_e.owner)) {
                    Some(Event::Build(BuildEvent {
                                          name: b_e.name,
                                          b_name: b_e.b_name,
                                          b_type: b_e.b_type.clone(),
                                          speed: 0.1,
                                          progress: 0.0,
                                          max_progress: 100.0,
                                      }))
                } else {
                    None
                }
            }
            Event::Move(m_e) => {
                if let Some(object) = self.get_object_mut(&m_e.name, None) {
                    if distance(object.x, object.y, object.drive_dest_x, object.drive_dest_y) <
                       object.drive_speed * elapsed {
                        object.x = object.drive_dest_x;
                        object.y = object.drive_dest_y;
                        None
                    } else {
                        if !((object.x - m_e.dest_x).abs() < ::FLOAT_ERR) {
                            if object.x < m_e.dest_x {
                                object.x += object.drive_speed * elapsed;
                            } else if object.x > m_e.dest_x {
                                object.x -= object.drive_speed * elapsed;
                            }
                        }
                        if !((object.y - m_e.dest_y).abs() < ::FLOAT_ERR) {
                            if object.y < m_e.dest_y {
                                object.y += object.drive_speed * elapsed;
                            } else if object.y > m_e.dest_y {
                                object.y -= object.drive_speed * elapsed;
                            }
                        }
                        Some(Event::Move(m_e))
                    }
                } else {
                    None
                }
            }
            Event::Destroy(d_e) => {
                if self.check_object_exsists(&d_e.name, None) {
                    self.objects.remove(&d_e.name);
                }
                None
            }
            Event::Damage(d_e) => {
                let mut events = vec![];
                for i in self.objects.iter_mut() {
                    let (_, mut object) = i;

                    if distance(object.x, object.y, d_e.x, d_e.y) <= d_e.size {
                        object.shell_damage(d_e.d_type.clone(), d_e.damage);
                        if object.shell_health <= 0.0 {
                            events.push(Event::Destroy(DestroyEvent { name: object.name.clone() }));
                        }
                    }
                }
                for ev in events {
                    self.add_event(ev);
                }
                None
            }
            Event::Build(b_e) => {
                if b_e.progress >= b_e.max_progress {
                    let result_object = {
                        if let Some(object) = self.get_object(&b_e.name, None) {
                            Some(SampleObject::new(object.owner.clone(),
                                                   b_e.b_name,
                                                   b_e.b_type,
                                                   object.x,
                                                   object.y))
                        } else {
                            None
                        }
                    };
                    if let Some(object) = result_object {
                        self.objects.insert(object.name.clone(), object).unwrap();
                    }
                    None
                } else {
                    let mut new_event = b_e.clone();
                    new_event.progress += b_e.speed * elapsed;
                    Some(Event::Build(new_event))
                }
            }
        };
        if let Some(ev) = return_event {
            self.add_event(ev);
        }
    }
}

pub fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt()
}
