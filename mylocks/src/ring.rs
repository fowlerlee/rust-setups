#[allow(unused_imports)]
use std::ops::{Deref, DerefMut};
#[allow(unused_imports)]
use std::{mem, thread};

#[allow(dead_code)]
struct InnerRing {
    capacity: isize,
    size: usize,
    data: *mut Option<u32>,
}

#[allow(dead_code)]
#[derive(Clone)]
struct Ring {
    inner: *mut InnerRing,
}

impl Ring {
    fn with_capacity(capacity: usize) -> Self {
        let mut data = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            data.push(None);
        }

        let raw_data = (&mut data).as_mut_ptr();
        mem::forget(data);
        let inner_ring = Box::new(InnerRing {
            capacity: capacity as isize,
            size: 0,
            data: raw_data,
        });

        Self {
            inner: Box::into_raw(inner_ring),
        }
    }
}

impl Deref for Ring {
    type Target = InnerRing;
    fn deref(&self) -> &InnerRing {
        unsafe { &*self.inner }
    }
}

impl DerefMut for Ring {
    fn deref_mut(&mut self) -> &mut InnerRing {
        unsafe { &mut *self.inner }
    }
}

fn writer(mut ring: Ring) -> () {
    let mut offset: isize = 0;
    let mut cur: u32 = 0;
    loop {
        unsafe {
            if (*ring).size != (*ring).capacity as usize {
                *(*ring).data.offset(offset) = Some(cur);
                (*ring).size += 1;
                cur = cur.wrapping_add(1);
                offset += 1;
                offset %= (*ring).capacity;
            } else {
                thread::yield_now()
            }
        }
    }
}
