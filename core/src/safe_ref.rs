#[cfg(not(feature = "thread-safe"))]
use std::cell::RefCell as Cell;

#[cfg(feature = "thread-safe")]
use std::sync::Mutex as Cell;

#[cfg(not(feature = "thread-safe"))]
pub use std::{
    cell::RefMut as Lock,
    rc::{Rc as Ref, Weak},
};

#[cfg(feature = "thread-safe")]
pub use std::sync::{Arc as Ref, MutexGuard as Lock, Weak};

#[repr(transparent)]
pub struct Mut<T: ?Sized>(Cell<T>);

impl<T> Mut<T> {
    pub fn new(inner: T) -> Self {
        Self(Cell::new(inner))
    }
}

impl<T: Default> Default for Mut<T> {
    fn default() -> Self {
        Mut::new(T::default())
    }
}

impl<T: ?Sized> Mut<T> {
    pub fn lock(&self) -> Lock<T> {
        #[cfg(not(feature = "thread-safe"))]
        {
            self.0.borrow_mut()
        }

        #[cfg(feature = "thread-safe")]
        {
            self.0.lock().unwrap()
        }
    }

    pub fn try_lock(&self) -> Option<Lock<T>> {
        #[cfg(not(feature = "thread-safe"))]
        {
            self.0.try_borrow_mut().ok()
        }

        #[cfg(feature = "thread-safe")]
        {
            self.0.lock().ok()
        }
    }
}
