// Game-wide constants
use device_query::Keycode;

//Rendering
pub const TARGET_FPS: u64 = 60;
pub const WINDOW_WIDTH: u64 = 40;
pub const WINDOW_HEIGHT: u64 = 20;

// Controls
#[derive(Debug)]
pub struct Controls {
    pub up: Keycode,
    pub down: Keycode,
    pub right: Keycode,
    pub left: Keycode
}
impl Controls {
    pub const fn new() -> Self {
        Self {
            up: Keycode::W,
            down: Keycode::S,
            right: Keycode::D,
            left: Keycode::A,
        }
    }
}

// Player
pub const PLAYER_CONTROLS: Controls = Controls::new();
pub const PLAYER_SPEED: i32 = 1;