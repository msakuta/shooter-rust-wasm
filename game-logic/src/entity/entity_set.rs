use std::cell::RefCell;

use super::{EntityId, RefMutOption, RefOption};

#[derive(Clone, PartialEq, Eq, Debug)]
/// An entry in entity list with generational ids, with the payload and the generation
pub struct EntityEntry<T> {
    pub gen: u32,
    pub payload: RefCell<Option<T>>,
}

impl<T> EntityEntry<T> {
    pub(crate) fn new(payload: T) -> Self {
        Self {
            gen: 0,
            payload: RefCell::new(Some(payload)),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntitySet<T> {
    v: Vec<EntityEntry<T>>,
}

impl<T> Default for EntitySet<T> {
    fn default() -> Self {
        Self { v: vec![] }
    }
}

impl<T> EntitySet<T> {
    pub fn new() -> Self {
        Self { v: vec![] }
    }

    pub fn clear(&mut self) {
        self.v.clear();
    }

    /// Returns the number of active elements in this EntitySet.
    /// It does _not_ return the buffer length.
    pub fn len(&self) -> usize {
        // TODO: optimize by caching active elements
        self.v
            .iter()
            .filter(|entry| entry.payload.borrow().is_some())
            .count()
    }

    /// Return an iterator over Ref<T>.
    /// It borrows the T immutably.
    pub fn iter(&self) -> impl Iterator<Item = RefOption<T>> {
        self.v.iter().filter_map(|v| RefOption::new(&v.payload))
    }

    /// Return an iterator over &mut T. It does not borrow the T with a RefMut,
    /// because the self is already exclusively referenced.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.v
            .iter_mut()
            .filter_map(|v| v.payload.get_mut().as_mut())
    }

    /// Return an iterator over RefMut<T>, skipping already borrowed items.
    /// It borrows the T mutablly.
    pub fn iter_borrow_mut(&self) -> impl Iterator<Item = RefMutOption<T>> {
        self.v.iter().filter_map(|v| RefMutOption::new(&v.payload))
    }

    /// Return an iterator over (id, Ref<T>)
    /// It is convenient when you want the EntityId of the iterated items.
    /// It borrows the T immutably.
    pub fn items(&self) -> impl Iterator<Item = (EntityId<T>, RefOption<T>)> {
        self.v.iter().enumerate().filter_map(|(i, v)| {
            Some((EntityId::new(i as u32, v.gen), RefOption::new(&v.payload)?))
        })
    }

    /// Return an iterator over (id, &mut T).
    /// It is convenient when you want the EntityId of the iterated items.
    /// It does not borrow the T with a RefMut, because the self is already exclusively referenced.
    pub fn items_mut(&mut self) -> impl Iterator<Item = (EntityId<T>, &mut T)> {
        self.v.iter_mut().enumerate().filter_map(|(i, v)| {
            Some((
                EntityId::new(i as u32, v.gen),
                v.payload.get_mut().as_mut()?,
            ))
        })
    }

    /// Return an iterator over (id, RefMut<T>), skipping already borrowed items.
    /// It is convenient when you want the EntityId of the iterated items.
    /// It borrows the T mutablly.
    pub fn items_borrow_mut(&self) -> impl Iterator<Item = (EntityId<T>, RefMutOption<T>)> {
        self.v.iter().enumerate().filter_map(|(i, v)| {
            Some((
                EntityId::new(i as u32, v.gen),
                RefMutOption::new(&v.payload)?,
            ))
        })
    }

    // pub fn split_mid(&self, idx: usize) -> Option<(Ref<T>, &[EntityEntry<T>], &[EntityEntry<T>])> {
    //     if self.v.len() <= idx {
    //         return None;
    //     }
    //     let (first, mid) = self.v.split_at(idx);
    //     let (center, last) = mid.split_first()?;
    //     Some((center.payload.as_ref()?.try_borrow().ok()?, first, last))
    // }

    // pub fn split_mid_mut(&mut self, idx: usize) -> Option<(&mut T, EntitySliceMut<T>)> {
    //     if self.v.len() <= idx {
    //         return None;
    //     }
    //     let (first, mid) = self.v.split_at_mut(idx);
    //     let (center, last) = mid.split_first_mut()?;
    //     Some((center.payload.as_mut()?.get_mut(), EntitySliceMut([first, last])))
    // }

    pub fn insert(&mut self, val: T) -> EntityId<T> {
        for (i, entry) in self.v.iter_mut().enumerate() {
            let payload = entry.payload.get_mut();
            if payload.is_none() {
                entry.gen += 1;
                entry.payload = RefCell::new(Some(val));
                return EntityId::new(i as u32, entry.gen);
            }
        }
        self.v.push(EntityEntry::new(val));
        EntityId::new(self.v.len() as u32 - 1, 0)
    }

    pub fn remove(&mut self, id: EntityId<T>) -> Option<T> {
        self.v.get_mut(id.id as usize).and_then(|entry| {
            if id.gen == entry.gen {
                entry.payload.get_mut().take()
            } else {
                None
            }
        })
    }

    pub fn retain(&mut self, mut f: impl FnMut(&mut T) -> bool) {
        for entry in &mut self.v {
            let Some(payload) = entry.payload.get_mut().as_mut() else {
                continue;
            };
            if !f(payload) {
                entry.payload = RefCell::new(None);
            }
        }
    }

    pub fn retain_id(&mut self, mut f: impl FnMut(EntityId<T>, &mut T) -> bool) {
        for (i, entry) in self.v.iter_mut().enumerate() {
            let Some(payload) = entry.payload.get_mut().as_mut() else {
                continue;
            };
            let id = EntityId::new(i as u32, entry.gen);
            if !f(id, payload) {
                entry.payload = RefCell::new(None);
            }
        }
    }

    pub fn retain_borrow_mut(&self, mut f: impl FnMut(&mut T, EntityId<T>) -> bool) {
        for (id, entry) in self.v.iter().enumerate() {
            let Ok(mut payload) = entry.payload.try_borrow_mut() else {
                continue;
            };
            if payload.is_none() {
                continue;
            }
            if !f(
                payload.as_mut().unwrap(),
                EntityId::new(id as u32, entry.gen),
            ) {
                *payload = None;
            }
        }
    }

    pub fn get(&self, id: EntityId<T>) -> Option<RefOption<T>> {
        self.v.get(id.id as usize).and_then(|entry| {
            if id.gen == entry.gen {
                RefOption::new(&entry.payload)
            } else {
                None
            }
        })
    }

    pub fn get_mut(&mut self, id: EntityId<T>) -> Option<&mut T> {
        self.v.get_mut(id.id as usize).and_then(|entry| {
            if id.gen == entry.gen {
                entry.payload.get_mut().as_mut()
            } else {
                None
            }
        })
    }

    /// Get without generation check
    pub fn get_mut_at(&mut self, idx: usize) -> Option<&mut T> {
        self.v
            .get_mut(idx)
            .and_then(|entry| entry.payload.get_mut().as_mut())
    }

    pub fn borrow_mut_at(&self, idx: usize) -> Option<RefMutOption<T>> {
        self.v
            .get(idx)
            .and_then(|entry| RefMutOption::new(&entry.payload))
    }
}

/// An inefficient (boxed) iterator for convenicence
impl<'a, T> IntoIterator for &'a EntitySet<T> {
    type Item = RefOption<'a, T>;
    type IntoIter = Box<dyn Iterator<Item = RefOption<'a, T>> + 'a>;
    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.iter()) as Box<_>
    }
}

/// An inefficient (boxed) mutable iterator for convenicence
impl<'a, T> IntoIterator for &'a mut EntitySet<T> {
    type Item = &'a mut T;
    type IntoIter = Box<dyn Iterator<Item = &'a mut T> + 'a>;
    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.iter_mut()) as Box<_>
    }
}
