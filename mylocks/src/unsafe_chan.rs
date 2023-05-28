use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    #[doc = r"Safety : unsafe send, can give undefined behaviour"]
    pub unsafe fn send(&self, message: T) {
        (*self.message.get()).write(message);
        self.ready.store(true, Release)
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Acquire)
    }

    #[doc = r"Safety : unsafe receive, can give undefined behaviour"]
    pub unsafe fn receive(&self) -> T {
        (*self.message.get()).assume_init_read()
    }
}
