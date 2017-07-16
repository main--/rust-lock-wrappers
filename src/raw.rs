pub trait Mutex {
    type LockState;

    fn lock(&self) -> Self::LockState;
    fn try_lock(&self) -> Option<Self::LockState>;

    fn unlock(&self, state: Self::LockState);
}

pub trait RwLock {
    type ReadLockState;
    type WriteLockState;

    fn acquire_read(&self) -> Self::ReadLockState;
    fn acquire_write(&self) -> Self::WriteLockState;
    fn release_read(&self, state: Self::ReadLockState);
    fn release_write(&self, state: Self::WriteLockState);
}
