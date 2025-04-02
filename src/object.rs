// Game engine structs
use crate::constants::{Controls, PLAYER_CONTROLS, PLAYER_SPEED, WINDOW_HEIGHT, WINDOW_WIDTH};
use device_query::{Keycode, DeviceState, DeviceQuery};

// Generic Object
// Specifies the position and sprite
// More complex logic is implemented in structs that have objects
#[derive(Clone, Debug)]
pub struct Object {
    // let player = Object::new(sprite: "V", pos: (50, 50))
    pub sprite: String,
    // pos = (x, y)
    pub pos: (i32, i32)
}

impl Object {
    pub fn new(sprite: String, pos: (i32, i32)) -> Self {
        Self {
            sprite,
            pos
        }
    }
}

// Movable Trait
pub trait Movable {
    fn move_pos(&mut self, delta: (i32, i32));
}

// Player object
#[derive(Debug)]
pub struct Player {
    pub object: Object,
    pub controls: Controls
}

impl Movable for Player {
    fn move_pos(&mut self, delta: (i32, i32)) {
        self.object.pos = (
            (self.object.pos.0 + delta.0).clamp(0,(WINDOW_WIDTH - 1) as i32),
            (self.object.pos.1 + delta.1).clamp(0, (WINDOW_HEIGHT - 1) as i32)
        )
    }
}

impl Player {
    pub fn new(sprite: String, pos: (i32, i32)) -> Self {
        Self {
            object: Object::new(sprite, pos),
            controls: PLAYER_CONTROLS
        }
    }
    pub fn apply_input(&mut self, input: Keycode) {
        // assuming input only moves
        let delta = match input {
            _ if input == self.controls.up => (0, -PLAYER_SPEED),
            _ if input == self.controls.down => (0, PLAYER_SPEED),
            _ if input == self.controls.right => (PLAYER_SPEED, 0),
            _ if input == self.controls.left => (-PLAYER_SPEED, 0),
            _ => (0, 0)
        };
        self.move_pos(delta);
    }
    pub fn test_update(&mut self) {
        let device_state = DeviceState::new();
        let keys = device_state.get_keys();
        let mut delta = (0, 0);

        if keys.contains(&self.controls.right) {
            delta.0 += PLAYER_SPEED
        } 
        if keys.contains(&self.controls.left) {
            delta.0 -= PLAYER_SPEED
        }
        if keys.contains(&self.controls.up) {
            delta.1 -= PLAYER_SPEED
        }
        if keys.contains(&self.controls.down) {
            delta.1 += PLAYER_SPEED
        }

        self.move_pos(delta);
    }
}

// Enemy
#[derive(Debug)]
pub struct Enemy {
    pub object: Object,
}

impl Enemy {
    pub fn new(sprite: String, pos: (i32, i32)) -> Self {
        Self {
            object: Object::new(sprite, pos),
       }
    }
}

impl Movable for Enemy {
    fn move_pos(&mut self, delta: (i32, i32)) {
        self.object.pos = (
            (self.object.pos.0 + delta.0).clamp(0, (WINDOW_WIDTH - 1) as i32),
            (self.object.pos.1 + delta.1).clamp(0, (WINDOW_HEIGHT - 1) as i32)
        )
    }
}



