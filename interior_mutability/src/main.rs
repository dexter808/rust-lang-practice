use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let v = Rc::new(RefCell::new(3));
    let a = Rc::new(Cons(Rc::clone(&v), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(2)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(1)), Rc::clone(&a));

    println!("a -> {:?}", a);
    println!("b -> {:?}", b);
    println!("c -> {:?}", c);

    *v.borrow_mut() = 5;

    println!("a -> {:?}", a);
    println!("b -> {:?}", b);
    println!("c -> {:?}", c);
}
