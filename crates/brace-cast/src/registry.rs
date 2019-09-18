use std::any::{Any, TypeId};
use std::collections::HashMap;

use inventory::collect;
use lazy_static::lazy_static;

use crate::Cast;

lazy_static! {
    static ref CAST_REF_REGISTRY: CastRefRegistry = CastRefRegistry::new();
    static ref CAST_MUT_REGISTRY: CastMutRegistry = CastMutRegistry::new();
}

collect!(CastRefRecord);
collect!(CastMutRecord);

pub type CastRefHandler<T> = fn(&dyn Any) -> Option<&T>;
pub type CastMutHandler<T> = fn(&mut dyn Any) -> Option<&mut T>;

pub fn cast_from_ref<S, T>(from: &S) -> Option<&T>
where
    S: Cast + ?Sized + 'static,
    T: ?Sized + 'static,
{
    CAST_REF_REGISTRY.cast_from_ref(from)
}

pub fn cast_from_mut<S, T>(from: &mut S) -> Option<&mut T>
where
    S: Cast + ?Sized + 'static,
    T: ?Sized + 'static,
{
    CAST_MUT_REGISTRY.cast_from_mut(from)
}

pub struct CastRefRecord(TypeId, TypeId, Box<dyn Any + Sync>);

impl CastRefRecord {
    pub fn new<S, T>(handler: CastRefHandler<T>) -> Self
    where
        S: 'static,
        T: ?Sized + 'static,
    {
        Self(TypeId::of::<T>(), TypeId::of::<S>(), Box::new(handler))
    }
}

pub struct CastMutRecord(TypeId, TypeId, Box<dyn Any + Sync>);

impl CastMutRecord {
    pub fn new<S, T>(handler: CastMutHandler<T>) -> Self
    where
        S: 'static,
        T: ?Sized + 'static,
    {
        Self(TypeId::of::<T>(), TypeId::of::<S>(), Box::new(handler))
    }
}

#[derive(Default)]
pub struct CastRefRegistry(HashMap<(TypeId, TypeId), &'static CastRefRecord>);

impl CastRefRegistry {
    pub fn new() -> Self {
        let mut map = HashMap::new();

        for rec in inventory::iter::<CastRefRecord> {
            map.insert((rec.0, rec.1), rec);
        }

        Self(map)
    }

    pub fn cast_from_ref<'a, S, T>(&self, from: &'a S) -> Option<&'a T>
    where
        S: Cast + ?Sized + 'static,
        T: ?Sized + 'static,
    {
        let from = from.cast_as_any_ref();
        let type_id = from.type_id();

        if let Some(rec) = self.0.get(&(TypeId::of::<T>(), type_id)) {
            let item = (&*rec.2) as &dyn Any;

            if let Some(cast) = item.downcast_ref::<CastRefHandler<T>>() {
                return (cast)(from);
            }
        }

        None
    }
}

#[derive(Default)]
pub struct CastMutRegistry(HashMap<(TypeId, TypeId), &'static CastMutRecord>);

impl CastMutRegistry {
    pub fn new() -> Self {
        let mut map = HashMap::new();

        for rec in inventory::iter::<CastMutRecord> {
            map.insert((rec.0, rec.1), rec);
        }

        Self(map)
    }

    pub fn cast_from_mut<'a, S, T>(&self, from: &'a mut S) -> Option<&'a mut T>
    where
        S: Cast + ?Sized + 'static,
        T: ?Sized + 'static,
    {
        let from = (*from).cast_as_any_mut();
        let type_id = (from as &dyn Any).type_id();

        if let Some(rec) = self.0.get(&(TypeId::of::<T>(), type_id)) {
            let item = (&*rec.2) as &dyn Any;

            if let Some(cast) = item.downcast_ref::<CastMutHandler<T>>() {
                return (cast)(from);
            }
        }

        None
    }
}
