use crate::{entities, utils::{
    constants::*,
    utils::*
}};
use std::{
        any::Any,
        cell::{RefCell, Ref, RefMut},
        fmt::{Debug, Formatter, Result as FmtResult, Write}
};

///
/// Components (C of ECS)
///
pub trait AnyComponent {
    fn into_box(self) -> Box<dyn AnyComponent>;
    fn insert_into_entity(self: Box<Self>, world: &mut World, entity: usize);
}

impl<T: 'static + Debug + Clone> AnyComponent for T {
    fn into_box(self) -> Box<dyn AnyComponent> {
        Box::new(self)
    }

    fn insert_into_entity(self: Box<Self>, world: &mut World, entity: usize) {
        world.add_component_to_entity(entity, *self);
    }
}
trait ComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult;
    fn fmt_entity(&self, entity: usize, f: &mut dyn Write) -> FmtResult;
}

impl Debug for dyn ComponentVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.fmt(f)
    }
}

impl<T: 'static + Debug> ComponentVec for RefCell<Vec<Option<T>>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.get_mut().push(None)
    }

    /// for Debug trait
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_list().entries(self.borrow().iter()).finish()
    }

    /// For formatting an entity
    fn fmt_entity(&self, entity: usize, f: &mut dyn std::fmt::Write) -> std::fmt::Result {
        if let Some(Some(comp)) = self.borrow().get(entity) {
            write!(f, "{:?}", comp)
        } else {
            Ok(())
        }
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


    /// Add component of ComponentType (e.g. Health, Position, etc.) to entity
    /// In case a ComponentVec for that ComponentType doesn't exist yet, create it.
    pub fn add_component_to_entity<ComponentType: 'static + Debug>(&mut self, entity: usize, component: ComponentType) {
        // find the correct component vec by trying to downcast each to ComponentType
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
            .as_any_mut()
            .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>() {
                component_vec.get_mut()[entity] = Some(component);
                return;
            }
        }

        // if ComponentVec doesn't exist in the world yet, add it
        let mut new_component_vec: Vec<Option<ComponentType>> = Vec::with_capacity(self.entities_count);
        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }
        new_component_vec[entity] = Some(component);
        self.component_vecs.push(Box::new(RefCell::new(new_component_vec)));
    }

    pub fn spawn(&mut self, components: Vec<Box<dyn AnyComponent>>) -> usize {
        // create empty entity
        let entity_id = self.entities_count;
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.entities_count += 1;

        // fill with specified components
        for component in components {
            component.insert_into_entity(self, entity_id);
        }
        entity_id
    }

    pub fn borrow_compvec_mut<ComponentType: 'static>(&self) -> Option<RefMut<Vec<Option<ComponentType>>>> {
        for compvec in self.component_vecs.iter() {
            if let Some(compvec) = compvec
            .as_any()
            .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>() {
                return Some(compvec.borrow_mut())
                }
        }
        None
    }

    pub fn borrow_compvec<ComponentType: 'static>(&self) -> Option<Ref<Vec<Option<ComponentType>>>> {
        for compvec in self.component_vecs.iter() {
            if let Some(compvec) = compvec
            .as_any()
            .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>() {
                return Some(compvec.borrow())
                }
        }
        None
    }

    /// print all components of entity
    pub fn debug_entity(&self, entity: &usize) {
        print!("Entity({}): [", entity);
        let mut first = true;
        for comp_vec in self.component_vecs.iter() {
            let mut output = String::new();
            let _ = comp_vec.fmt_entity(*entity, &mut output);

            if !output.is_empty() {
                if !first {
                    print!(", ");
                }
                print!("{}", output);
                first = false;
            }
        }
        println!("]");
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
        self.x = (self.x + delta.0).clamp(0,(WINDOW_WIDTH - 1) as i32);
        self.y = (self.y + delta.1).clamp(0,(WINDOW_HEIGHT - 1) as i32);
    }
}

#[derive(Debug, Clone)]
pub struct Velocity {
    pub x: i32,
    pub y: i32
}

impl Velocity {

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn add(&mut self, delta: (i32, i32)) {
        // TODO: temp, the clamping should be handled by a CollisionHandler
        self.x += delta.0;
        self.y += delta.1;
    }
    pub fn zero(&mut self) {
        self.x = 0;
        self.y = 0;
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
