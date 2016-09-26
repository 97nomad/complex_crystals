#[derive(RustcDecodable, RustcEncodable, Clone)]
pub enum ObjectType {
    Harvester,
    Battlecruiser,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub enum RadarType {
    Simple,
    Middle,
    Military,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub enum WeaponType {
    Mining,
    Laser,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub enum CargoType {
    Mining,
    Battery,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub enum ArmorType {
    Asteroid,
    Light,
    Middle,
    Heavy,
    Building,
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct SampleObject {
    pub owner: String,
    pub name: String,
    pub otype: ObjectType,
    pub x: f64,
    pub y: f64,

    pub drive_speed: f64,
    pub drive_dest_x: f64,
    pub drive_dest_y: f64,

    pub radar_radius: f64,
    pub radar_type: RadarType,

    pub weapon_active: bool,
    pub weapon_type: WeaponType,
    pub weapon_radius: f64,
    pub weapon_target_x: f64,
    pub weapon_target_y: f64,

    pub cargo_type: CargoType,
    pub cargo_max: f64,
    pub cargo_current: f64,

    pub shell_health: f64,
    pub shell_type: ArmorType,
}