// Part 1:
// ----------------------------------------------
// Given a binary tree, write a function that visits each node once.

// typedef struct Node {
//     Node* left;
//     Node* right;
//     int value;
// } Node;
use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
    #[allow(unused)]
    lca: Option<Rc<RefCell<Node>>>,
    val: i32,
    is_root: bool,
}

impl Node {
    fn new(val: i32) -> Self {
        Self {
            left: None,
            right: None,
            lca: None,
            val,
            is_root: false,
        }
    }

    fn set_left(&mut self, left: Node) {
        self.left = Some(Rc::new(RefCell::new(left)));
    }

    fn set_right(&mut self, right: Node) {
        self.right = Some(Rc::new(RefCell::new(right)));
    }

    #[allow(dead_code)]
    fn set_lca(&mut self, node: Rc<RefCell<Node>>) {
        // when visited set its lca
        if node.borrow_mut().is_root {
            return;
        }
        // TODO: fix this part
        // node.borrow_mut().lca =
    }
}

//          3
//        /   \
//       5     1
//      / \   / \
//     6   2  0  8
//        / \
//       7   4

#[allow(unconditional_recursion)]
fn visit(root: Rc<RefCell<Node>>) {
    println!("{:?}", root.borrow().val);
    if let Some(x) = root.borrow().left.clone() {
        visit(x);
    }
    if let Some(x) = root.borrow().right.clone() {
        visit(x);
    }
}

fn main() {
    let mut root = Node::new(3);
    let mut five = Node::new(5);
    let two = Node::new(2);
    let six = Node::new(6);
    five.set_left(six);
    five.set_right(two);
    root.set_left(five);
    root.is_root = true;
    visit(Rc::new(RefCell::new(root)));
}
