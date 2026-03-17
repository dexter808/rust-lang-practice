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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let q = "duct";
        let cnt = "\
Rust: 
Safe fast productive.
pick three.
        ";
        assert_eq!(vec!["Safe fast productive."], search(q, cnt));
    }
}