use crate::components::components::{
    Position,
    Velocity
};
use std::cell::{RefCell, Ref, RefMut};

pub fn advance_position_system(
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