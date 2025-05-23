// Utility functions

use device_query::{DeviceQuery, DeviceState, Keycode};

pub fn clear_screen(){
    std::process::Command::new("clear").status().unwrap();
}
pub fn read_input() -> Option<Keycode> {
    let device_state = DeviceState::new();
    // while keys.len() == 0 {
    //     keys = device_state.get_keys();
    // }
    let keys = device_state.get_keys();
    if keys.len() != 0 {
        Some(keys[0])
    } else {
        None
    }
}

pub fn pos_rel_to_abs(pos: (f32, f32), height: usize, width: usize) -> (usize, usize) {
    // Converts relative [0.0, 1.0] positions into absolute screen coordinates.
    let scaled_x = (pos.0 * ((width-1) as f32)).round();
    let scaled_y = (pos.1 * ((height-1) as f32)).round();
    let result = (scaled_x as usize, scaled_y as usize);
    result
}