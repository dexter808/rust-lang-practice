use std::{cell::RefCell, rc::Rc};
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None
        }
    }
}

fn main() {
    let a = Rc::new(Cons(1, RefCell::new(Rc::new(Nil))));

    println!("a inital rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", &a.tail());

    let b = Rc::new(Cons(2, RefCell::new(Rc::clone(&a))));

    println!("b inital rc count = {}", Rc::strong_count(&b));
    println!("a next item = {:?}", b.tail());

    if let Some(item) = a.tail() {
        *item.borrow_mut() = Rc::clone(&b);
    }

    println!("a final rc count = {}", Rc::strong_count(&a));
    println!("b final rc count = {}", Rc::strong_count(&b));

    // print!("Print reference cycle a -> {:?}", a);

}
