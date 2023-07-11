use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::AtomicU8,
    sync::atomic::Ordering::{Acquire, Relaxed, Release},
};

const EMPTY: u8 = 0;
const WRITING: u8 = 1;
const READY: u8 = 2;
const READING: u8 = 3;

pub struct Channy<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    state: AtomicU8,
}

unsafe impl<T: Send> Sync for Channy<T> {}

impl<T> Channy<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            state: AtomicU8::new(EMPTY),
        }
    }

    pub fn send(&self, message: T) {
        if self
            .state
            .compare_exchange(EMPTY, WRITING, Relaxed, Relaxed)
            .is_err()
        {
            panic!("can't send more than one message!")
        }
        unsafe {
            (*self.message.get()).write(message);
        }
        self.state.store(READY, Release);
    }

    pub fn is_ready(&self) -> T {
        if self
            .state
            .compare_exchange(READY, READING, Acquire, Relaxed)
            .is_err()
        {
            panic!("no message available")
        }
        unsafe { (*self.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channy<T> {
    fn drop(&mut self) {
        if *self.state.get_mut() == READY {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_unsafe_channy() {
       let ch: Channy<String>  = Channy::new();
       ch.send("letter".to_string());
    }

    #[test]
    fn test_channy() {}
}
