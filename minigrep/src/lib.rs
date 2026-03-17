use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("File Content: {}",contents);
    return Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
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