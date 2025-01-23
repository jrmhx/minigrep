use std::fs;
use std::error::Error;

#[derive(Debug)]
pub struct Config{
    file_path: String,
    query: String,
    ignore_case: bool
}

impl Config {
    pub fn build(
            mut args: impl Iterator<Item = String>
        ) -> Result<Self, &'static str> {
        
        args.next(); // skip the first arg as the cli paramater always has a first arg as binary's path

        let file_path = match args.next() {
            Some(x) => x,
            None => return Err("Didn't get a file path"),
        };

        let query = match args.next() {
            Some(x) => x,
            None => return Err("Didn't get a query"),
        };

        let ignore_case = std::env::var("IGNORE_CASE")
            .is_ok_and(|x|{x == "1"});

        Ok(
            Config {
                file_path,
                query,
                ignore_case
            }
        )
    }
}

pub fn run(config: & Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(&config.file_path)?;
    println!("content:\n{}", content);

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    println!("\nfound:");
    result.iter().for_each(|l| println!("{}", l));

    Ok(())
}

fn search<'a> (query: &str, content: &'a str) -> Vec<&'a str> {
    let mut ans = Vec::new();
    for l in content.lines() {
        if l.contains(query) {
            ans.push(l);
        }
    }

    ans
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut ans = Vec::new();
    let query = query.to_lowercase();

    for l in content.lines() {
        if l.to_lowercase().contains(&query) {
            ans.push(l);
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_result(){
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn test_case_insensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search_case_insensitive(query, content));
    }
}
