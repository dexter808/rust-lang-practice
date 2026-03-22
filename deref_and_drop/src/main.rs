use std::{fmt::Display, ops::Deref};

struct MyBox<T: Display>(T);

impl<T: Display> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: Display> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(x, *y);
    assert_eq!(x, *(y.deref()));
    assert_eq!(5, x);

    let m = MyBox::new(String::from("How are you - 1 !!"));
    let m2 = MyBox::new(String::from("How are you - 2 !!"));
    drop(&m);
    hello(&m); // &MyBox<String> -> &String -> &str
    hello(&(*m)[..]);
    println!("Last Line of main");
}


fn hello(name: &str) {
    println!("Hello, {}", name);
}