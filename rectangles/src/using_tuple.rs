pub fn using_tuple() {
    let rect :(u32, u32) = (5, 10);
    println!("Area of {:?} is -> {}", rect, area(rect));
}

fn area(rect: (u32,u32)) -> u32 {
    rect.0 * rect.1
}