extern crate piston;
extern crate graphics;
extern crate piston_window;
extern crate find_folder;
extern crate time;

// #[cfg(feature="piston")]
#[macro_use]
extern crate conrod;

extern crate rand;
extern crate rustc_serialize;
extern crate hyper;

mod network;
mod engine;
pub mod scenes;
mod utils;
mod ui;
mod server;
mod level_generator;
mod data_types;

const FLOAT_ERR: f64 = std::f64::EPSILON;

fn main() {
    ::engine::spawn();
}
