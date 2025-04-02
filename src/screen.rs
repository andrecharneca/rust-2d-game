// Screen implementation

use crate::utils::{pos_rel_to_abs, clear_screen};
use crate::object::Object;

pub struct Screen {
    width: usize,
    height: usize,
    filler_str: String,
    grid: Vec<Vec<String>>,
}

impl Screen {
    // Initialize with blank screen
    pub fn new(width: usize, height: usize) -> Self {
        let filler_str = " ".to_string();
        Self {
            width,
            height,
            filler_str: filler_str.clone(),
            grid: vec![vec![filler_str; width]; height]
        }
    }

    // Reset grid to default vals
    pub fn reset_grid(&mut self) {
        self.grid = vec![vec![self.filler_str.clone(); self.width]; self.height];
    }

    // Update the grid with objects
    pub fn update_grid(&mut self, objects: &Vec<Object>) {
        self.reset_grid();
        for obj in objects {
            let abs_pos = pos_rel_to_abs(obj.pos, self.height, self.width);
            self.grid[abs_pos.1][abs_pos.0] = obj.sprite.clone();
        }
    }

    // Render grid as screen with border
    pub fn render(&self) {
        clear_screen();
        println!("{}", vec!["▁".to_string(); self.width+1].concat());
        for hline in &self.grid {
            println!("▏{}▏", hline.concat());
        }
        println!("{}", vec!["▔".to_string(); self.width+1].concat());
    }
}