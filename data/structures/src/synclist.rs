use std::cell::{Ref, RefCell};
use std::collections::LinkedList;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct Node {
    value: String,
    next: Link,
    prev: Link,
}

type Link = Option<Arc<Mutex<Node>>>;

impl Node {
    fn new(value: String) -> Arc<Mutex<Node>> {
        Arc::new(Mutex::new(Node {
            value: (value),
            next: (None),
            prev: (None),
        }))
    }
}

unsafe impl Sync for Node {}
unsafe impl Send for Node {}

#[derive(Clone)]
pub struct Logger {
    head: Link,
    tail: Link,
    pub length: u64,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            head: (None),
            tail: (None),
            length: (0),
        }
    }

    pub fn append(&mut self, value: String) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => {
                old.borrow_mut().next = Some(new.clone());
                new.borrow_mut().prev = Some(old);
            }
            None => self.head = Some(new.clone()),
        }
        self.length += 1;
        self.tail = Some(new)
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                next.borrow_mut().prev = None;
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Error occurred when popping off list")
                .into_inner()
                .value
        })
    }
}

// pub struct ListIterator {
//     current: Link,
// }

// impl ListIterator {
//     fn new(start_at: Link) -> Self {
//         Self { current: start_at }
//     }
// }

// impl IntoIterator for Logger {
//     type Item = String;
//     type IntoIter = ListIterator;

//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }
