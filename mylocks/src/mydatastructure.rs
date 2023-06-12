use std::boxed::Box;
use std::collections::hash_map::DefaultHasher;
use std::mem;

#[derive(Default, Clone)]
pub struct Notification {
    alert: String,
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

    pub fn is_more_mail_present(&self) -> bool {
        true
    }

    pub fn add(&mut self, note: Notification) {
        self.heap.push(Box::new(note));
        self.size = self.heap.len();
    }
}

#[cfg(test)]
mod tests {
    use super::{HeapOfMail, Notification};

    #[test]
    #[ignore]
    fn it_works() {
        let note1 = Notification::new();
        let note2 = Notification::new();

        let mut new = HeapOfMail::new_empty();
        new.add(note1);
        new.add(note2);
    }
}
