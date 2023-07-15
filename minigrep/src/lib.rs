use std::error::Error;
use std::{env, fs};
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("命令行参数小于3");
        } else {
            let ignore_case = env::var("IGNORE_CASE").is_ok();
            Ok(Config {
                query: args[1].clone(),
                file_path: args[2].clone(),
                ignore_case,
            })
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    let vec = if config.ignore_case {
        search_insensitive_case(&config.query, &content)
    } else {
        search(&config.query, &content)
    };
    for line in vec {
        println!("{}", line)
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line.trim())
        }
    }
    return res;
}

pub fn search_insensitive_case<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line.trim());
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
       Rust:
        safe, fast, productive.
                    Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "Duct";
        let contents = "\
       Rust:
        safe, fast, productive.
                    Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search_insensitive_case(query, contents)
        );
    }
}
