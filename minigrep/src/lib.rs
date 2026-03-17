use std::error::Error;
use std::{fs, env};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    // println!("File Content: {}",contents);

    let result = if config.case_sensitivity {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    return Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitivity: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            Err("not enough arguments provided")
        } else {
            let q = &args[1];
            let f = &args[2];
            let ci = env::var("RUST_TEST_CASE_INSENSITIVE").is_err();
        
            Ok(
                Config {
                    query: q.clone(),
                    filename: f.clone(),
                    case_sensitivity: ci,
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
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

    #[test]
    fn case_insensitive() {
        let q = "rust";
        let cnt = "\
Rust: 
Safe fast productive.
pick three.
Trust me  .
        ";
        assert_eq!(vec!["Rust: ", "Trust me  ."], search_case_insensitive(q, cnt));
    }
}
