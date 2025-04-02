// Game engine structs
use crate::constants::{Controls, PLAYER_CONTROLS, PLAYER_SPEED};
use device_query::Keycode;

// Generic Object
// Specifies the position and sprite
// More complex logic is implemented in structs that have objects
#[derive(Clone, Debug)]
pub struct Object {
    // let player = Object::new(sprite: "V", pos: (0.5, 0.5))
    pub sprite: String,
    // pos = (x, y)
    pub pos: (f32, f32)
}

impl Object {
    pub fn new(sprite: String, pos: (f32, f32)) -> Self {
        Self {
            sprite,
            pos
        }
    }
}

// Movable Trait
pub trait Movable {
    fn move_pos(&mut self, delta: (f32, f32));
}

// Player object
#[derive(Debug)]
pub struct Player {
    pub object: Object,
    pub controls: Controls
}

impl Movable for Player {
    fn move_pos(&mut self, delta: (f32, f32)) {
        self.object.pos = (
            (self.object.pos.0 + delta.0).clamp(0.0, 1.0),
            (self.object.pos.1 + delta.1).clamp(0.0, 1.0)
        )
    }
}

impl Player {
    pub fn new(sprite: String, pos: (f32, f32)) -> Self {
        Self {
            object: Object::new(sprite, pos),
            controls: PLAYER_CONTROLS
        }
    }
    pub fn apply_input(&mut self, input: Keycode) {
        // assuming input only moves
        let delta = match input {
            _ if input == self.controls.up => (0.0, -PLAYER_SPEED),
            _ if input == self.controls.down => (0.0, PLAYER_SPEED),
            _ if input == self.controls.right => (PLAYER_SPEED, 0.0),
            _ if input == self.controls.left => (-PLAYER_SPEED, 0.0),
            _ => (0.0, 0.0)
        };
        self.move_pos(delta);
    }
}

// Enemy
#[derive(Debug)]
pub struct Enemy {
    pub object: Object,
}

impl Enemy {
    pub fn new(sprite: String, pos: (f32, f32)) -> Self {
        Self {
            object: Object::new(sprite, pos),
       }
    }
}

impl Movable for Enemy {
    fn move_pos(&mut self, delta: (f32, f32)) {
        self.object.pos = (
            (self.object.pos.0 + delta.0).clamp(0.0, 1.0),
            (self.object.pos.1 + delta.1).clamp(0.0, 1.0)
        )
    }
}



