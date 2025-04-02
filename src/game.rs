use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use rand::Rng;
use crate::utils::read_input;
use crate::object::{Movable, Player, Enemy};
use crate::screen::Screen;
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH, TARGET_FPS};
use device_query::Keycode;

// Main loop
pub fn run_game() {
    let mut screen = Screen::new(WINDOW_WIDTH as usize, WINDOW_HEIGHT as usize);
    let mut player = Player::new("P".to_string(), (WINDOW_WIDTH as i32 / 2, WINDOW_HEIGHT as i32 / 2));
    let mut enemy = Enemy::new("E".to_string(), (5, 5));

    // Arc mutex to handle async player input
    // let input_shared = Arc::new(Mutex::new(None::<Keycode>));
    // let input_clone = Arc::clone(&input_shared);

    // // Spawn user input thread
    // thread::spawn(move || {
    //     loop {
    //         let input = read_input();
    //         let mut input_lock = input_clone.lock().unwrap();
    //         *input_lock = Some(input);
    //     }
    // });

    // Game loop
    let frame_duration = Duration::from_millis(1000 / TARGET_FPS as u64);
    let mut last_frametime = Instant::now();
    let mut frametime_history = vec![last_frametime.elapsed()];
    loop {
        let objects = vec![enemy.object.clone(), player.object.clone()];

        screen.update_grid(&objects);
        screen.render();

        println!("{:?}", player);
        println!("Last frametime: {:?}", frametime_history.pop().unwrap());

        // Check user input thread for a value
        // let mut input_lock = input_shared.lock().unwrap();
        // if let Some(input) = *input_lock {
        //     println!("Input: {:?}", input);
        //     player.apply_input(input);
        //     *input_lock = None;
        // } else {
        //     println!("No input...")
        // }
        player.test_update();
        enemy.move_pos((
            rand::thread_rng().gen_range(-5..5) as i32,
            rand::thread_rng().gen_range(-5..5) as i32,
        ));
        
        // Guarantee consistent frametime
        let elapsed = last_frametime.elapsed();
        if elapsed < frame_duration {
            thread::sleep(frame_duration - elapsed);
        }

        frametime_history.push(last_frametime.elapsed());
        last_frametime = Instant::now();
    }
}
