use std::ops::Add;

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
    fn fly(&self) -> ();
}


trait Wizard {
    fn fly(&self) -> ();
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("*Waving hands furiously*");
    }
}

impl Pilot for Human {
    fn fly(&self) -> () {
        println!("*Uses a plane to fly*");
    }
}

impl Wizard for Human {
    fn fly(&self) -> () {
        println!("*Uses broom to fly*");
    }
}


fn main() {
    assert_eq!(Point{x:3, y: 2} + Point{x: 2, y: 3}, Point{x: 5, y: 5});
    assert_eq!(Millimeters(12) + Meter(2), Millimeters(2012));

    let me = Human;
    me.fly();
    Wizard::fly(&me);
    Pilot::fly(&me);
}
