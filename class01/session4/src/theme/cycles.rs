// IKinder

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

pub fn main() {
    crate::show_name(file!());

    let tail = Rc::new(Node {
        value: 1,
        next: RefCell::new(NextNode::None),
    });
    let head = Rc::new(Node {
        value: 2,
        next: RefCell::new(NextNode::Strong(tail.clone())),
    });
    *tail.next.borrow_mut() = NextNode::Weak(Rc::downgrade(&head));
    println!("Head: {head:?}")
}

#[derive(Debug)]
struct Node {
    value: i32,
    // next: RefCell<Option<Rc<NodeBad>>>,
    next: RefCell<NextNode>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping node with {} value", self.value)
    }
}

#[derive(Debug)]
enum NextNode {
    None,
    Strong(Rc<Node>),
    Weak(Weak<Node>),
}
