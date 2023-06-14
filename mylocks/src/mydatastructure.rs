extern crate env_logger;
extern crate uuid;
extern crate zookeeper;

use std::boxed::Box;
use std::mem;
use uuid::*;

#[allow(unused_imports)]
use std::{env, sync::Arc, thread, time::Duration};

#[allow(unused_imports)]
use zookeeper::{recipes::leader::LeaderLatch, WatchedEvent, Watcher, ZooKeeper};

#[derive(Default, Clone)]
pub struct Notification {
    alert: String,
}

//create watcher
#[derive(Default, Clone, Debug)]
pub struct MailWatcher;

impl Watcher for MailWatcher {
    fn handle(&self, event: WatchedEvent) {
        print!("Watcher handling event: {:?} ", event)
    }
}

pub fn create_zookeeper() -> ZooKeeper {
    let addr = "localhost:2181";
    ZooKeeper::connect(addr, Duration::from_secs(3), MailWatcher).unwrap()
}

pub fn create_znode(path: &str, zk: ZooKeeper) {
    let data = vec![0..255u8];
    // zk.create(path, data, acl, mode)
}

pub fn create_latch(zk: ZooKeeper) -> LeaderLatch {
    let id = Uuid::new_v4().to_string();
    const LATCH_PATH: &str = "/latch-ex";

    LeaderLatch::new(Arc::new(zk), id, LATCH_PATH.into())
}

#[derive(Default)]
pub struct HeapOfMail {
    pub size: usize,
    heap: Vec<Box<Notification>>,
}

impl Notification {
    pub fn new() -> Self {
        Self {
            alert: Default::default(),
        }
    }
}

impl HeapOfMail {
    pub fn new_empty() -> Self {
        Self {
            size: 0usize,
            heap: vec![],
        }
    }

    pub fn swap(&mut self, pos1: usize, pos2: usize) {
        let m2 = self.heap[pos1 - 1].clone();
        self.heap[pos1 - 1] = mem::replace(&mut self.heap[pos2 - 1], m2);
    }

    pub fn is_more_mail_present(&self, pos1: usize, pos2: usize) -> bool {
        let a = &self.heap[pos1 - 1];
        let b = &self.heap[pos2 - 1];
        a.alert >= b.alert
    }

    pub fn add(&mut self, note: Notification) {
        self.heap.push(Box::new(note));
        self.size = self.heap.len();

        if self.size > 1 {
            let mut i = self.size;
            while i / 2 > 0 && self.is_more_mail_present(i, i / 2) {
                self.swap(i, i / 2);
                i /= 2;
            }
        }
    }

    pub fn pop(&mut self) {
        // if self.size > 0 {
        //     let elem = self.heap.swap_remove(0);
        //     self.size = self.heap.len();
        //     let mut i = 1;
        //     while i * 2 < self.size {
        //         let children = (i * 2, i * 2 + 1);
        //         i = if self.is_more_mail_present(children.0, children.1) {
        //             if self.is_more_mail_present(children.0, i) {
        //                 self.swap(i, children.0);
        //                 children.0
        //             } else {
        //                 break;
        //             }
        //         } else {
        //             if self.is_more_mail_present(children.1, i) {
        //                 self.swap(i, children.1);
        //                 children.1
        //             } else {
        //                 break;
        //             }
        //         }
        //     }
        //     Some(*elem)
        // } else {
        //     None
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_notes() {
        let note1 = Notification::new();
        let note2 = Notification::new();

        let mut new = HeapOfMail::new_empty();
        new.add(note1);
        new.add(note2);
    }

    #[test]
    fn test_zookeeper() {
        let zk = create_zookeeper();
        let latch = create_latch(zk).start().unwrap();
        println!("ran test");
    }

    #[test]
    #[ignore]
    fn createZnodeShould() {}
}
