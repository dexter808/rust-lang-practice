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

struct Millimeters(u32);

struct Meter(u32);

impl Add<Meter> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meter) -> Self::Output {
        Millimeters (rhs.0 * 1000 + self.0)
    }
}

fn main() {
    assert_eq!(Point{x:3, y: 2} + Point{x: 2, y: 3}, Point{x: 5, y: 5})
}
