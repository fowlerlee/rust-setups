use std::boxed::Box;
use std::collections::hash_map::DefaultHasher;
use std::mem;

#[derive(Default)]
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
            heap: Default::default(),
        }
    }

    pub fn swap(&self) {
        unimplemented!()
    }

    pub fn is_more_mail_present(&self) -> bool {
        unimplemented!()
    }

    pub fn add(&self) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::HeapOfMail;

    #[test]
    #[ignore]
    fn it_works() {
        let new = HeapOfMail::new_empty();
        new.add();
        if new.is_more_mail_present() {
            new.swap();
        }
    }
}
