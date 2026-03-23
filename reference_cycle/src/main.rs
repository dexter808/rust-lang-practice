use std::{cell::RefCell, rc::{Rc, Weak}};

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!("Strong count leaf: {}, Weak count leaf: {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("Strong count branch: {}, Weak count branch: {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
    
    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
}
