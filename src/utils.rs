use std::ops::{Deref, DerefMut};

/// This crate's wrapper type.
pub struct W<T>(pub T);

impl<T> Deref for W<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for W<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for W<T> {
    fn from(value: T) -> Self {
        W(value)
    }
}