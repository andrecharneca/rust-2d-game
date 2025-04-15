// for debugging
pub fn run_game() {
    use crate::components::*;
    let mut world = World::new();
    let player = world.new_entity();
    world.add_component_to_entity(player, Health(100));
    world.add_component_to_entity(player, Position{x: 10, y:10});
    println!("{:#?}", world);
}



// Main loop
pub fn run_game_1() {
    use std::{
        thread,
        time::{Duration, Instant},
    };
    use crate::entity::{Entity, Player, Enemy};
    use crate::screen::Screen;
    use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH, TARGET_FPS};
    let mut screen = Screen::new(WINDOW_WIDTH as usize, WINDOW_HEIGHT as usize);
    let mut player = Player::new(
        "P".to_string(),
        WINDOW_WIDTH as i32 / 2,
        WINDOW_HEIGHT as i32 / 2
    );
    let mut enemy = Enemy::new(
        "E".to_string(),
        10,
        10
    );
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
        let entities = vec![Entity::Player(player.clone()), Entity::Enemy(enemy.clone())];
        
        screen.update_grid(entities);
        screen.render();

        println!("{:?}", player);
        println!("{:?}", enemy);
        println!("Last frametime: {:?}", frametime_history.pop().unwrap());

        // Check user input thread for a value
        // let mut
        player.update();
        enemy.update();
        
        // Guarantee consistent frametime
        let elapsed = last_frametime.elapsed();
        if elapsed < frame_duration {
            thread::sleep(frame_duration - elapsed);
        }

        frametime_history.push(last_frametime.elapsed());
        last_frametime = Instant::now();
    }
}
