use rand::Rng;
use std::cmp::Ordering;

use std::io;

fn main() {
    println!("Guess The Number Game!! ");

    let secret_number :u8 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess the number. Enter the number when you are ready!!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readLine");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number");
                continue
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
            Ordering::Greater => {println!("Too Big");},
            Ordering::Less => println!("Too Small")
        };
    }
}
