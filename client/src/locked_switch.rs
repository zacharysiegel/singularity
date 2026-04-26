use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug)]
pub struct LockedSwitch<T: Copy> {
    pub current: RwLock<T>,
    pub next: RwLock<Option<T>>,
}

impl<T: Copy> LockedSwitch<T> {
    pub const fn new(initial: T) -> Self {
        Self {
            current: RwLock::new(initial),
            next: RwLock::new(None),
        }
    }

    pub fn register_next(&self, inner: T) {
        let mut next: RwLockWriteGuard<Option<T>> = self.next.write().unwrap();
        *next = Some(inner);
    }

    /// Returns (previous, current). Previous is None if no state change occurred.
    pub fn update(&self) -> (Option<T>, T) {
        let next: RwLockReadGuard<Option<T>> = self.next.read().unwrap();
        if next.is_none() {
            let current: RwLockReadGuard<T> = self.current.read().unwrap();
            return (None, *current);
        }
        drop(next);

        let mut current: RwLockWriteGuard<T> = self.current.write().unwrap();
        let mut next: RwLockWriteGuard<Option<T>> = self.next.write().unwrap();

        let previous: T = *current;
        *current = next.as_ref().unwrap().clone();
        *next = None;

        (Some(previous), *current)
    }
}
