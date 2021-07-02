/// for illustration purposes only, do not use IRL!
use std::cell::RefCell; // a "runtime borrow-checker"
use std::fmt::Debug;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node<T> {
    parent: Option<Weak<RefCell<Node<T>>>>,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
    value: T,
}

impl<T: Debug> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            parent: None,
            left: None,
            right: None,
            value,
        }
    }

    fn print_parent(&self) {
        if let Some(parent) = &self.parent {
            if let Some(parent) = parent.upgrade() {
                println!("Parent: {:?}", parent.borrow().value);
            } else {
                println!("Parent: no more parent");
            }
        } else {
            println!("Parent: None");
        }
    }
}

fn main() {
    let root = Rc::new(RefCell::new(Node::new(0)));
    let node = Rc::new(RefCell::new(Node::new(1)));
    node.borrow_mut().parent = Some(Rc::downgrade(&root));

    let node2 = Rc::new(RefCell::new(Node::new(2)));

    root.borrow_mut().parent = Some(Rc::downgrade(&node2));
    root.borrow_mut().left = Some(node);

    println!("Root:   {:?}", *root.borrow());
    if let Some(left) = &root.borrow().left {
        &left.borrow().print_parent();
    };
    &root.borrow().print_parent();
    drop(node2);
    &root.borrow().print_parent();
}
