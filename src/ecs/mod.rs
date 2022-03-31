use std::collections::HashMap;
use std::any::Any;

mod cell;
use cell::TrustCell;

#[derive(PartialEq, Eq, Hash)]
pub struct ResourceId(u32);

pub trait Resource : Any + Send + Sync + 'static {}

pub struct Entity {
    generation: u32,
    id: u32,
}

#[derive(Default)]
pub struct World {
    entities: Vec<Entity>,
    resources: HashMap<ResourceId, TrustCell<Box<dyn Resource>>>,
}

impl World {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn insert<R>(&mut self, id: ResourceId, r: R)
    where
        R: Resource,
    {
        self.resources.insert(id, TrustCell::new(Box::new(r)));
    }

    pub fn remove(&mut self, id: ResourceId) -> Option<Box<dyn Resource>>
    {
        self.resources
            .remove(&id)
            .map(TrustCell::into_inner)
            .map(|x: Box<dyn Resource>| x)
    }

    pub fn has_value(&self, id: ResourceId) -> bool
    {
        self.resources.contains_key(&id)
    }
}
