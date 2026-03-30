use std::{fmt::Display, ops::Add};

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

#[derive(PartialEq, Debug)]
struct Millimeters(u32);


#[derive(PartialEq, Debug)]
struct Meter(u32);

impl Add<Meter> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meter) -> Self::Output {
        Millimeters (rhs.0 * 1000 + self.0)
    }
}

trait Pilot {
    fn fly() -> ();
}


trait Wizard {
    fn fly() -> ();
}

struct Human;

impl Human {
    fn fly() {
        println!("*Waving hands furiously*");
    }
}

impl Pilot for Human {
    fn fly() -> () {
        println!("*Uses a plane to fly*");
    }
}

impl Wizard for Human {
    fn fly() -> () {
        println!("*Uses broom to fly*");
    }
}


// Super traits
trait OutlinePrint : Display {
    fn outline_print(&self) {
        let output: String = self.to_string();
        let len = output.len();
        println!("Len -> {}, Output -> {}", len, output);
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

// New type pattern

struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[My Display function on vectors -> {}]", self.0.join(", "))
    }
}

fn main() {
    assert_eq!(Point{x:3, y: 2} + Point{x: 2, y: 3}, Point{x: 5, y: 5});
    assert_eq!(Millimeters(12) + Meter(2), Millimeters(2012));

    let me = Human;
    Human::fly();
    // Fully qualified syntax
    <Human as Wizard>::fly();
    <Human as Pilot>::fly();

    let p = Point{x: 2, y: 3};
    p.outline_print();

    let w = Wrapper(vec![String::from("1 - Hello"), String::from("2 - World")]);
    println!("Wrapper -> {}", w);
}
