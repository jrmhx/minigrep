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
    println!("\x1b[32m\ncontent:\x1b[0m");
    println!("{}", content);

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };
    let q_size = config.query.len();
    let query = if config.ignore_case {
        &config.query.to_lowercase()
    } else {
        &config.query
    };
    println!("\x1b[32m\nfound:\x1b[0m");

    // print with colored terminal log high light
    result.iter().for_each(|line| {
        let mut l = 0;

        let search_line = match config.ignore_case {
            true => &line.to_lowercase(),
            false => &line.to_string()
        };
        
        while l < line.len() {
            let r = search_line[l..]
                .find(query)
                .or_else(|| Some(line.len()))
                .unwrap();
            if r < line.len() {
                print!("{}", &line[l..l+r]);
                warning_log(&line[l+r..l+r+q_size]);
                l += r + q_size;
            } else {
                print!("{}", &line[l..]);
                l = r;
            }
        }

        print!("\n");
    });

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

fn warning_log(s: &str) {
    print!("\x1b[93m{s}\x1b[0m");
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
