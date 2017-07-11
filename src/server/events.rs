use data_types::{ObjectType, WeaponType};

pub enum Event {
    MoveRequest(NetworkMoveEvent),
    FireRequest(NetworkFireEvent),
    BuildRequest(NetworkBuildEvent),
    Move(MoveEvent),
    Destroy(DestroyEvent),
    Damage(DamageEvent),
    Build(BuildEvent),
}

pub struct NetworkMoveEvent {
    pub name: String,
    pub owner: String,
    pub dest_x: f64,
    pub dest_y: f64,
}

pub struct NetworkFireEvent {
    pub name: String,
    pub owner: String,
    pub dest_x: f64,
    pub dest_y: f64,
}

pub struct NetworkBuildEvent {
    pub name: String,
    pub owner: String,
    pub b_type: ObjectType,
    pub b_name: String,
}

#[derive(Clone)]
pub struct MoveEvent {
    pub name: String,
    pub dest_x: f64,
    pub dest_y: f64,
}

pub struct DestroyEvent {
    pub name: String,
}

pub struct DamageEvent {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub d_type: WeaponType,
    pub damage: f64,
}

#[derive(Clone)]
pub struct BuildEvent {
    pub name: String,
    pub b_name: String,
    pub b_type: ObjectType,
    pub speed: f64,
    pub progress: f64,
    pub max_progress: f64,
}
