use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use raw::RwLock as RawRwLock;

pub struct RwLock<L: RawRwLock, T> {
    rwlock: L,
    data: UnsafeCell<T>,
}

unsafe impl<L: RawRwLock, T: Send + Sync> Send for RwLock<L, T> {}
unsafe impl<L: RawRwLock, T: Send + Sync> Sync for RwLock<L, T> {}

impl<L: RawRwLock, T> RwLock<L, T> {
    pub fn new(l: L, t: T) -> RwLock<L, T> {
        RwLock {
            rwlock: l,
            data: UnsafeCell::new(t),
        }
    }

    #[inline]
    pub fn read(&self) -> RwLockReadGuard<L, T> {
        RwLockReadGuard {
            state: Some(self.rwlock.acquire_read()),
            rwlock: self,
        }
    }

    #[inline]
    pub fn write(&self) -> RwLockWriteGuard<L, T> {
        RwLockWriteGuard {
            state: Some(self.rwlock.acquire_write()),
            rwlock: self,
        }
    }
}

impl<L: RawRwLock + Default, T: Default> Default for RwLock<L, T> {
    fn default() -> RwLock<L, T> {
        RwLock::new(Default::default(), Default::default())
    }
}

#[must_use]
pub struct RwLockReadGuard<'a, L: RawRwLock + 'a, T: 'a> {
    state: Option<L::ReadLockState>,
    rwlock: &'a RwLock<L, T>,
}

#[must_use]
pub struct RwLockWriteGuard<'a, L: RawRwLock + 'a, T: 'a> {
    state: Option<L::WriteLockState>,
    rwlock: &'a RwLock<L, T>,
}

impl<'a, L: RawRwLock + 'a, T: 'a> Drop for RwLockReadGuard<'a, L, T> {
    fn drop(&mut self) {
        self.rwlock.rwlock.release_read(self.state.take().unwrap());
    }
}

impl<'a, L: RawRwLock + 'a, T: 'a> Drop for RwLockWriteGuard<'a, L, T> {
    fn drop(&mut self) {
        self.rwlock.rwlock.release_write(self.state.take().unwrap());
    }
}

impl<'a, L: RawRwLock + 'a, T: 'a> Deref for RwLockReadGuard<'a, L, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.rwlock.data.get() }
    }
}

impl<'a, L: RawRwLock + 'a, T: 'a> Deref for RwLockWriteGuard<'a, L, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.rwlock.data.get() }
    }
}

impl<'a, L: RawRwLock + 'a, T: 'a> DerefMut for RwLockWriteGuard<'a, L, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.rwlock.data.get() }
    }
}
