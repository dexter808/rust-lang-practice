use std::env;
use std::error::Error;
use std::{fs, process};

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

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("File Content: {}",contents);
    return Ok(())
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