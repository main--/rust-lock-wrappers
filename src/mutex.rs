use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use raw::Mutex as RawMutex;

pub struct Mutex<L: RawMutex, T> {
    mutex: L,
    data: UnsafeCell<T>,
}

unsafe impl<L: RawMutex, T: Send> Send for Mutex<L, T> { }
unsafe impl<L: RawMutex, T: Send> Sync for Mutex<L, T> { }

impl<L: RawMutex, T> Mutex<L, T> {
    pub fn new(l: L, t: T) -> Mutex<L, T> {
        Mutex {
            mutex: l,
            data: UnsafeCell::new(t),
        }
    }

    pub fn lock(&self) -> MutexGuard<L, T> {
        MutexGuard {
            state: Some(self.mutex.lock()),
            mutex: self,
        }
    }

    pub fn try_lock(&self) -> Option<MutexGuard<L, T>> {
        self.mutex.try_lock().map(|x| MutexGuard {
            state: Some(x),
            mutex: self,
        })
    }
}

impl<L: RawMutex + Default, T: Default> Default for Mutex<L, T> {
    /// Creates a `Mutex<T>`, with the `Default` value for T.
    fn default() -> Mutex<L, T> {
        Mutex::new(Default::default(), Default::default())
    }
}

#[must_use]
pub struct MutexGuard<'a, L: RawMutex + 'a, T: 'a> {
    state: Option<L::LockState>,
    mutex: &'a Mutex<L, T>,
}

impl<'a, L: RawMutex + 'a, T: 'a> Drop for MutexGuard<'a, L, T> {
    fn drop(&mut self) {
        self.mutex.mutex.unlock(self.state.take().unwrap());
    }
}

impl<'a, L: RawMutex + 'a, T: 'a> Deref for MutexGuard<'a, L, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, L: RawMutex + 'a, T: 'a> DerefMut for MutexGuard<'a, L, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}
