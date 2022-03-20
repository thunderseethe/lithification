use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;

mod cell;
use cell::TrustCell;

pub struct ResourceId(u32);

pub trait Resource : Any + Send + Sync + 'static {}

#[derive(Default)]
pub struct World {
    resources: HashMap<ResourceId, TrustCell<Box<dyn Resource>>>,
}

impl World {
    pub fn empty() -> Self {
        Default::default()
    }

    pub fn insert<R>(&mut self, id: ResourceId, r: R)
    where
        R: Resource,
    {
        self.resources.insert(id, TrustCell::new(Box::new(r)));
    }

    pub fn remove<R>(&mut self, id: ResourceId) -> Option<R> 
    where
        R: Resource,
    {
        self.resources
            .remove(&id)
            .map(TrustCell::into_inner)
            .map(|x: Box<dyn Resource>| x.downcast())
            .map(|x: Result<Box<R>, _>| x.ok().unwrap())
            .map(|x| *x)
    }

    pub fn has_value(&self, id: ResourceId) -> bool
    {
        self.resources.contains_key(&id)
    }
}
