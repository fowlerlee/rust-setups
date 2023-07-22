use std::collections::HashMap;


type Link = Box<Node>;

struct Node {
    pub key: i128,
    next: HashMap<i128, Link>,
    pub value: Option<Document>
}

#[derive(Clone, Debug)]
pub struct Document {
    pub numerical_id: u64,
    pub path: String,
    pub address: String,
}

impl Document {
    pub fn new(id: u64, address: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            numerical_id: id,
            address: address.into(),
            path: path.into(),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }
}
