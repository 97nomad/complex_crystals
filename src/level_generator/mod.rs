mod diamond_square;

use std::sync::{Arc, Mutex};
use server::server::GameEngine;
use data_types::ObjectType;

pub fn generate(mutex: Arc<Mutex<GameEngine>>,
                map_width: f64,
                map_height: f64,
                players: Vec<String>) {
    let noise_size = clp2((((map_width + map_height) / 2.0).sqrt()) as usize) as usize + 1;
    println!("Размер шума - {}", noise_size);
    let coeff_width = map_width / noise_size as f64;
    let coeff_height = map_height / noise_size as f64;

    print!("Генерация астероидов... ");
    let asteroid_noise = diamond_square::generate_ds(noise_size);
    let mut debug_asteroid_count = 0;

    let mut engine = mutex.lock().unwrap();

    for y in 0..noise_size {
        for x in 0..noise_size {
            let level = asteroid_noise[x][y].powf(2.0);
            if level > 1.0 {
                engine.add_object(format!("Asteroid{}{}", &*x.to_string(), &*y.to_string()),
                                  x as f64 * coeff_width,
                                  y as f64 * coeff_height,
                                  ObjectType::Asteroid,
                                  "unknown".to_owned());
                debug_asteroid_count += 1;
            }
        }
    }
    println!("{} Завершена", debug_asteroid_count);

    for (i, player) in players.iter().enumerate() {
        let mut basename = player.clone();
        basename.push_str("Base");
        engine.add_object(basename.to_owned(),
                          100.0,
                          100.0 * i as f64,
                          ObjectType::Builder,
                          player.clone());
    }
}

fn clp2(number: usize) -> usize {
    let x = number - 1;
    let x = x | (x >> 1);
    let x = x | (x >> 2);
    let x = x | (x >> 4);
    let x = x | (x >> 8);
    let x = x | (x >> 16);
    x + 1
}
