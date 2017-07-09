pub mod config;
pub mod events;
pub mod server;

pub use server::server::GameEngine;

use level_generator;

use std::sync::{Arc, Mutex};
use std::thread;
use time;

fn spawn(wsize_width: f64, wsize_height: f64, players: Vec<String>) {
    let mutex_engine = Arc::new(Mutex::new(server::GameEngine::new(wsize_width, wsize_height)));

    level_generator::generate(mutex_engine.clone(), wsize_width, wsize_height, players);

    let cloned_engine = mutex_engine.clone();
    //thread::spawn(move || network_engine::start(cloned_engine));

    let interval = 1_000_000_000 / 60;
    let mut before = time::precise_time_ns();
    let mut last_second = time::precise_time_ns();
    let mut tps = 0u16;

    loop {
        let mut engine = mutex_engine.lock().unwrap();
        let now = time::precise_time_ns();
        let dt = now - before;
        let elapsed = dt as f64 / 1_000_000.0;

        if dt < interval {
            thread::sleep(time::Duration::milliseconds(((interval - dt) / 1_000_000) as i64)
                              .to_std()
                              .unwrap());
            //continue 'running;
        }

        before = now;
        tps += 1;

        if now - last_second > 1_000_000_000 {
            last_second = now;
            engine.update_tps(tps);
            tps = 0;
        }

        engine.game_loop(elapsed);
    }
}
