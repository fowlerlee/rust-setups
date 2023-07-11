mod channy;

use std::sync::atomic::AtomicBool;
use std::sync::atomic::*;
use std::sync::mpsc::{Receiver, Sender};
use std::{sync::mpsc, thread};

unsafe impl<T> Send for SwapMutex<T> {}
unsafe impl<T> Sync for SwapMutex<T> {}

pub struct SwapMutex<T> {
    locked: AtomicBool,
    data: *mut T,
}

impl<T> SwapMutex<T> {
    fn new(t: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: Box::into_raw(Box::new(t)),
        }
    }
}

fn main() {
    let s = SwapMutex::new(1u32);

    let t = thread::spawn(move || {
        match s
            .locked
            .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
        {
            Ok(x) => {
                println!("result Ok: {:?}", x);
            }
            Err(y) => println!("result Err: {:?}", y),
        }
    });
}

#[cfg(test)]
mod tests {
    use std::cell::UnsafeCell;

    use super::*;

    #[test]
    #[ignore]
    fn test_channel() {
        let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
        let handles = (0..10).map(|i| {
            let tx2 = tx.clone();
            thread::spawn(move || {
                tx2.send(i).expect("could not send");
            })
        });
        for h in handles {
            h.join().unwrap();
        }
        let mut v: Vec<i32> = Vec::with_capacity(10);
        v.push(rx.recv().unwrap());
    }

    #[test]
    fn test_spinlock() {
        let s = SwapMutex::new(1u32);

        let t = thread::spawn(move || {
            match s
                .locked
                .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
            {
                Ok(x) => {
                    println!("result Ok: {:?}", x);
                }
                Err(y) => println!("result Err: {:?}", y),
            }
        });
    }
}
