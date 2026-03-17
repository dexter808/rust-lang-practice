use std::env;
use std::process;

use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err: &str | {
        println!("Problem occurred while parsing argumenst : {}", err);
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("in file : {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    };
}