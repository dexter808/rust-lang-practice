use std::env;
use std::{fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err: &str | {
        println!("Problem occurred while parsing argumenst : {}", err);
        process::exit(1);
    });
    
    println!("Query: {}", config.query);
    println!("Filepath: {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Failed to read fileru");

    println!("File Content: {}",contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            Err("not enough arguments provided")
        } else {
            let q = &args[1];
            let f = &args[2];
        
            Ok(
                Config {
                    query: q.clone(),
                    filename: f.clone(),
                }
            )
        }
    }
}