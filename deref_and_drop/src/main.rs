use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(x, *y);
    assert_eq!(x, *(y.deref()));
    assert_eq!(5, x);

    let m = MyBox::new(String::from("How are you !!"));
    hello(&m); // &MyBox<String> -> &String -> &str
    hello(&(*m)[..]);
}


fn hello(name: &str) {
    println!("Hello, {}", name);
}