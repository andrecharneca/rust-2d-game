use std::{
    thread,
    time::{Duration, Instant},
};
use crate::{entities::entities::*, systems::movement::advance_position_system};
use crate::systems::{
    movement,
    screen
};
use crate::utils::{
    constants::*,
    utils::*
};
use crate::systems::screen::*;
use crate::components::components::*;



// Main loop
pub fn run_game() {
    // World creation
    let mut world = World::new();
    let player = world.spawn(vec![
        Health(100).into_box(),
        Position{x: 10, y:10}.into_box(),
    ]);
    let enemy = world.spawn(vec![
        Health(50).into_box(),
    ]);

    // Debug
    println!("{:#?}", world);
    let health_compvec = world.borrow_compvec_mut::<Health>().unwrap();
    println!("{:?}", health_compvec);

    // Game loop
    let frame_duration = Duration::from_millis(1000 / TARGET_FPS as u64);
    let mut last_frametime = Instant::now();
    let mut frametime_history = vec![last_frametime.elapsed()];
    loop {
        // screen.update_grid(entities);
        // screen.render();

        println!("Last frametime: {:?}", frametime_history.pop().unwrap());
        
        world.debug_entity(&player);
        world.debug_entity(&enemy);
        advance_position_system(
            world.borrow_compvec_mut::<Position>().unwrap(),
            world.borrow_compvec::<Velocity>().unwrap()
        );

        // Guarantee consistent frametime
        let elapsed = last_frametime.elapsed();
        if elapsed < frame_duration {
            thread::sleep(frame_duration - elapsed);
        }

        frametime_history.push(last_frametime.elapsed());
        last_frametime = Instant::now();
    }
}


// // for debugging
// pub fn run_game_1() {
//     // World creation
//     let mut world = World::new();
//     let player = world.spawn(vec![
//         Health(100).into_box(),
//         Position{x: 10, y:10}.into_box(),
//     ]);
//     let enemy = world.spawn(vec![
//         Health(50).into_box(),
//     ]);

//     // Debug
//     println!("{:#?}", world);
//     world.debug_entity(player);
//     world.debug_entity(enemy);
//     let health_compvec = world.borrow_compvec_mut::<Health>().unwrap();
//     println!("{:?}", health_compvec);
// }

