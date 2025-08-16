#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn using_structs() {
    let rect = Rectangle {
        width: 5,
        height: 10,
    };

    println!("Area of {:#?} is {}", rect, area(&rect));
}

// As a general rule, we always use reference for data on heap / compound data
fn area(rect : &Rectangle) -> u32 {
    rect.width * rect.height
}