use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    
    println!("Query: {}", config.query);
    println!("Filepath: {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Failed to read file");

    println!("File Content: {}",contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let q = &args[1];
        let f = &args[2];
    
        Config {
            query: q.clone(),
            filename: f.clone(),
        }
    }
}