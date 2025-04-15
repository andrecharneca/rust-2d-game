use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use std::fmt::{Debug, Formatter, Result as FmtResult};


///
/// Components (C of ECS)
///
///
trait ComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult;
}

impl Debug for dyn ComponentVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.fmt(f)
    }
}

impl<T: 'static + Debug> ComponentVec for Vec<Option<T>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.push(None)
    }

    // for Debug trait
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_list().entries(self.iter()).finish()
    }
}

#[derive(Debug)]
pub struct World {
    entities_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>
}

impl World {
    pub fn new() -> Self {
        Self {
            entities_count: 0,
            component_vecs: Vec::new()
        }
    }

    pub fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.entities_count += 1;
        entity_id
    }

    /// Add component of ComponentType (e.g. Health, Position, etc.) to entity
    /// In case a ComponentVec for that ComponentType doesn't exist yet, create it.
    pub fn add_component_to_entity<ComponentType: 'static + Debug>(&mut self, entity: usize, component: ComponentType) {
        // find the correct component vec by trying to downcast each to ComponentType
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<Vec<Option<ComponentType>>>() {
                    component_vec[entity] = Some(component);
                    return;
                }
        }

        // if ComponentVec doesn't exist in the world yet, add it
        let mut new_component_vec: Vec<Option<ComponentType>> = Vec::with_capacity(self.entities_count);
        for _ in 0..self.entities_count {
            new_component_vec.push_none();
        }
        new_component_vec[entity] = Some(component);
        self.component_vecs.push(Box::new(new_component_vec));
    }
}


#[derive(Debug, Clone)]
pub struct Health(pub i32);


#[derive(Debug, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Position {
    pub fn new (x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn add(&mut self, delta: (i32, i32)) {
        // TODO: temp, the clamping should be handled by a CollisionHandler
        self.x = (self.x + delta.0).clamp(0,(WINDOW_WIDTH - 1) as i32);
        self.y = (self.y + delta.1).clamp(0,(WINDOW_HEIGHT - 1) as i32);
    }
}

#[derive(Debug, Clone)]
pub struct Velocity {
    pub x: f32,
    pub y: f32
}

impl Velocity {

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn add(&mut self, delta: (f32, f32)) {
        // TODO: temp, the clamping should be handled by a CollisionHandler
        self.x += delta.0;
        self.y += delta.1;
    }
    pub fn zero(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }
}


#[derive(Debug, Clone)]
pub struct Sprite {
    sprite: String
}

impl Sprite {
    pub fn new(sprite_str: String) -> Self {
        Self {sprite: sprite_str}
    }

    pub fn to_string(&self) -> String {
        self.sprite.clone()
    }
}
