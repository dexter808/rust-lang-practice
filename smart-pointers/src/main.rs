#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

use List::Cons;

fn main() {
    let b = Box::new("Hello");
    println!("{}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(List::Nil))))));
    println!("{:?}", list);
}