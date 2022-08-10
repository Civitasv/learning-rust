use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // parent can own children, but child won't own parent
    children: RefCell<Vec<Rc<Node>>>,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{borrow::Borrow, cell::RefCell, rc::Rc};
    #[test]
    fn test_tree() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("leaf paren = {:?}", leaf.parent.borrow().upgrade());
    }
}
