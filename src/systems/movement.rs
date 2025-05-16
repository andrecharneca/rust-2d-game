use core::panic;
use std::cell::{RefCell, Ref, RefMut};
use device_query::{Keycode};
use crate::components::components::{
    Position,
    Velocity
};
use crate::utils::constants::{
    PLAYER_SPEED,
    PLAYER_CONTROLS
};


pub fn advance_position_sys(
    mut positions: RefMut<Vec<Option<Position>>>,
    velocities: Ref<Vec<Option<Velocity>>>
) {
    for (a, b) in positions.iter_mut().zip(velocities.iter()) {
        if let (Some(p), Some(v)) = (a, b) {
            p.x += v.x;
            p.y += v.y;
        }
    }
}

pub fn advance_velocity_sys(
    mut velocities: RefMut<Vec<Option<Velocity>>>,
    input: &Option<Keycode>
) {
    let mut input_key = Keycode::Q;
    if let Some(ik) = *input {
        println!("Input detected in advance_velocity_sys");
        input_key = ik;
    } else {
        println!("No input detected in advance_velocity_sys");
        return
    }
    // set velocity based on input
    let v_new = match input_key {
        _ if input_key == PLAYER_CONTROLS.up => Velocity{x:0, y:-PLAYER_SPEED},
        _ if input_key == PLAYER_CONTROLS.down => Velocity{x:0, y:PLAYER_SPEED},
        _ if input_key == PLAYER_CONTROLS.right => Velocity{x:PLAYER_SPEED, y:0},
        _ if input_key == PLAYER_CONTROLS.left => Velocity{x:-PLAYER_SPEED, y:0},
        _ => Velocity{x:0, y:0}
    };

    // Add deltas
    for a in velocities.iter_mut() {
        if let Some(v) = a {
            *v = v_new;
        }
    }
}