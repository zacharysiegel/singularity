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

    pub fn update(&self) {
        let next: RwLockReadGuard<Option<T>> = self.next.read().unwrap();
        if next.is_none() {
            return;
        }
        drop(next);

        let mut current: RwLockWriteGuard<T> = self.current.write().unwrap();
        let mut next: RwLockWriteGuard<Option<T>> = self.next.write().unwrap();

        *current = next.as_ref().unwrap().clone();
        *next = None;
    }
}
