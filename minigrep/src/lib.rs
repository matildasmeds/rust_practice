use std::str::*;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Expected command line arguments: query_string filename");
        }
        let query = String::from_str(&args[1]).unwrap(); // another option would be clone
        let filename = String::from_str(&args[2]).unwrap();
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_config_new() {
        let mut args = vec![String::from("doesn't count")];
        let config = Config::new(&args);
        assert!(config.is_err());

        args.push(String::from("oh hai"));
        let config = Config::new(&args);
        assert!(config.is_err());

        args.push(String::from("lolcat.filez"));
        let config = Config::new(&args).unwrap();
        assert_eq!(config.query, args[1]);
        assert_eq!(config.filename, args[2]);
    }

    #[test]
    fn test_search() {
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let query = "duct";
        assert_eq!(
            search(query, contents),
            vec!["safe, fast, productive."]
        );

        let query = "Kotlin";
        assert!(search(query, contents).is_empty());

        let query = "st";
        assert_eq!(
            search(query, contents),
            vec!["Rust:", "safe, fast, productive."]
        );
    }
}
