// Game engine structs
use crate::constants::{Controls, PLAYER_CONTROLS, PLAYER_SPEED, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::components::{Position, Velocity, Sprite};
use device_query::{Keycode, DeviceState, DeviceQuery};
use rand::Rng;

// Generic Entity
pub enum Entity {
    Player(Player),
    Enemy(Enemy)
}
impl Entity {
    pub fn get_sprite(&self) -> String {
        match self {
           Entity::Player(p) => p.sprite.to_string(),
           Entity::Enemy(e) => e.sprite.to_string()
        }
    }
    pub fn get_pos(&self) -> (i32, i32) {
            match self {
            Entity::Player(p) => (p.pos.x, p.pos.y),
            Entity::Enemy(e) => (e.pos.x, e.pos.y)
            }
        }
}

// Player object
#[derive(Debug, Clone)]
pub struct Player {
    pub pos: Position,
    pub sprite: Sprite,
    pub controls: Controls
}

impl Player {
    pub fn new(sprite_str: String, x: i32, y: i32) -> Self {
        Self {
            pos: Position::new(x, y),
            sprite: Sprite::new(sprite_str),
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
        self.pos.add(delta);
    }
    
    /// Updates player position based on input
    pub fn update(&mut self) {
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

        self.pos.add(delta);
    }
}

// Enemy
#[derive(Debug, Clone)]
pub struct Enemy {
    pub pos: Position,
    pub vel: Velocity,
    pub sprite: Sprite,
}

impl Enemy {
    pub fn new(sprite_str: String, x: i32, y: i32) -> Self {
        Self {
            pos: Position::new(x, y),
            vel: Velocity::new(0.0, 0.0),
            sprite: Sprite::new(sprite_str),
        }
    }

    /// Update position with specific movement pattern
    pub fn update(&mut self) {
        // let delta = match self.pos.x >= (WINDOW_WIDTH-1) as i32 {
        //     true => (-1, 0),
        //     false => (1, 0)
        // };
        if self.pos.x >= (WINDOW_WIDTH-1) as i32 {
            self.pos.x = (WINDOW_WIDTH-1) as i32 / 2;
            self.vel.zero();
        }
        if self.pos.y >= (WINDOW_HEIGHT-1) as i32 {
            self.pos.y = (WINDOW_HEIGHT-1) as i32 / 2;
            self.vel.zero();
        }
        let delta_v = (
            rand::thread_rng().gen_range(-0.1..0.2),
            rand::thread_rng().gen_range(-0.1..0.2)
            );
        self.vel.add(delta_v);
        self.pos.add((self.vel.x as i32, self.vel.y as i32));
    }
}