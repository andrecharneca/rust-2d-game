use std::{thread::sleep, time::Duration};

mod game;
mod systems;
mod components;
mod utils;
mod entities;

fn main() {
    game::run_game();
}

// fn main() {
//     use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};
//     use std::thread;

//     let device_state = DeviceState::new();

//     loop {
//         let mouse: MouseState = device_state.get_mouse();
//         println!("Current Mouse Coordinates: {:?}", mouse.coords);
    
//         let keys: Vec<Keycode> = device_state.get_keys();
//         if keys.len() > 0 {
//             println!("Keys {:?}", keys);
//         }
//         std::thread::sleep(Duration::from_millis(1000));
//     }

// }