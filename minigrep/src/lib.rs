use std::error::Error;
use std::fs;
use std::result::Result;
use std::env;

pub struct Config {
    pub query: String, 
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        // If we want to use a command line argument too,
        // we can check args[3]
        // We'll need to make sure the args has a length of 4 first though.
        // Also we'd need to determine which takes precedence over the other
        // command line vs env var.
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case})
    } 
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
   
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents) 
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut matches = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            matches.push(line);
        }
    }

    matches
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut matches = vec![];
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            matches.push(line);
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
