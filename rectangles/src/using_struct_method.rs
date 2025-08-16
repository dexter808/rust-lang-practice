
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect :&Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

// each struct can have multiple impl block
impl Rectangle {
    // Example of an associated function, which will be called using '::'.
    fn square(size :u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn using_struct_method() {
    let rect = Rectangle {
        width: 5,
        height: 10,
    };
    println!("The area of {:?} is {}", rect, rect.area());

    let rect1 = Rectangle {
        width: 6,
        ..rect
    };
    let rect2 = Rectangle {
        width: 4,
        height: 5,
    };

    println!("Can {:?} hold rect1 {:?}? {}", rect, rect1, rect.can_hold(&rect1));
    println!("Can {:?} hold rect2 {:?}? {}", rect, rect2, rect.can_hold(&rect2));
    println!("Square as a rectangle: {:?}", Rectangle::square(5));
}