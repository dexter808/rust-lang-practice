use add_one;
use rand;
use add_two;

fn main() {
    let n = 10;
    println!("Using add-one function => {}", add_one::add_one(n));
    println!("Using add-one function => {}", add_two::add_two(n));
}
