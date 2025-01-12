use std::{cell::{Ref, RefCell, RefMut}, ops::{Deref, DerefMut}};

#[derive(Debug)]
/// A wrapper around Ref<Option> that always has Some.
/// We need a Ref to release the refcounter, but we would never return
/// a Ref(None).
pub struct RefOption<'a, T>(Ref<'a, Option<T>>);

impl<'a, T> RefOption<'a, T> {
    pub(super) fn new(val: &'a RefCell<Option<T>>) -> Option<Self> {
        let v = val.try_borrow().ok()?;
        if v.is_none() {
            return None;
        }
        Some(Self(v))
    }
}

impl<'a, T> Deref for RefOption<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}

/// A wrapper around RefMut<Option> that always has Some.
/// We need a RefMut to release the refcounter, but we would never return
/// a RefMut(None).
pub struct RefMutOption<'a, T>(RefMut<'a, Option<T>>);

impl<'a, T> RefMutOption<'a, T> {
    pub(super) fn new(val: &'a RefCell<Option<T>>) -> Option<Self> {
        let v = val.try_borrow_mut().ok()?;
        if v.is_none() {
            return None;
        }
        Some(Self(v))
    }
}

impl<'a, T> Deref for RefMutOption<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}

impl<'a, T> DerefMut for RefMutOption<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().unwrap()
    }
}
