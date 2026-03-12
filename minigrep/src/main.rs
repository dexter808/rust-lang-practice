use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let q = &args[1];
    let f = &args[2];
    
    println!("Querry: {}", *q);
    println!("Filepath: {}", f);

    let contents = fs::read_to_string(f).expect("Failed to read file");

    println!("File Content: {}",contents);
}
