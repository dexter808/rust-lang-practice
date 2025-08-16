fn main() {
    let s :String = String::from("Hello");
    let s1: &String = &s;
    let len = s1.len();
    println!("Length of {s1} is {len}");
}
