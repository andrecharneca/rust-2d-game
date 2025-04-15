// Screen implementation
use crate::utils::{pos_rel_to_abs, clear_screen};
use crate::entity::{self, Entity};

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
    pub fn update_grid(&mut self, entities: Vec<Entity>) {
        self.reset_grid();
        for entity in entities {
            let position = entity.get_pos();
            self.grid[position.1 as usize][position.0 as usize] = entity.get_sprite();
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