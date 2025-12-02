use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    dbg!("content:");
    dbg!("{}", &content);
    let result = if config.case_sensitive {
        search(&config.q, &content)
    } else {
        search_case_insensitive(&config.q, &content)
    };

    dbg!("result:");
    for line in result {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub filename: String,
    pub q: String,
    case_sensitive: bool,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        Ok(Config {
            filename: args[2].clone(),
            q: args[1].clone(),
            case_sensitive: env::var("CASE_SEN").is_ok(),
        })
    }
}

pub fn search<'a>(q: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(q) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(q: &str, content: &'a str) -> Vec<&'a str> {
    let q = q.to_lowercase();
    let mut result = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&q) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let q = "duc";
        let content = "something
rust productive
and idk
Duc tape
bye";
        assert_eq!(vec!["rust productive"], search(q, content));
    }

    #[test]
    fn case_insensitive() {
        let q = "Duc";
        let content = "something
rust productive
and DuCk
bye";

        assert_eq!(
            vec!["rust productive", "and DuCk"],
            search_case_insensitive(q, content)
        );
    }
}
