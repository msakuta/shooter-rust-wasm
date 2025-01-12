use std::{fmt::Display, marker::PhantomData};

use super::RefOption;

pub struct EntityId<T> {
    pub(super) id: u32,
    pub(super) gen: u32,
    _ph: PhantomData<fn(T)>,
}

impl<T> std::clone::Clone for EntityId<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            gen: self.gen,
            _ph: self._ph,
        }
    }
}

impl<T> std::marker::Copy for EntityId<T> {}

impl<T> std::fmt::Debug for EntityId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EntityId({}, {})", self.id, self.gen)
    }
}

impl<T> std::cmp::PartialEq for EntityId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.gen == other.gen
    }
}

impl<T> std::cmp::Eq for EntityId<T> {}

impl<T> std::hash::Hash for EntityId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.gen.hash(state);
    }
}

impl<T> EntityId<T> {
    pub(super) fn new(id: u32, gen: u32) -> Self {
        Self {
            id,
            gen,
            _ph: PhantomData,
        }
    }
}

impl<T> Display for EntityId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.id, self.gen)
    }
}

/// An extension trait to allow a container to iterate over valid items
pub trait EntityIterExt<T> {
    /// Iterate items in each entry's payload
    #[allow(dead_code)]
    fn items<'a>(&'a self) -> impl Iterator<Item = RefOption<'a, T>>
    where
        T: 'a;
}
