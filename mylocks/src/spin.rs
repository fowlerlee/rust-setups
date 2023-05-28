use std::cell::UnsafeCell;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::sync::atomic::{AtomicBool};

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> &T {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
        unsafe { &mut *self.value.get() }
    }

    #[doc = r"Safety : unsafe unlocking of the lock, can give undefined behaviour"]
    pub unsafe fn unlock(&self) {
        self.locked.store(false, Release)
    }
}
