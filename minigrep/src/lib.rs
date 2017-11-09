use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    // called with std::env::args() from main
    // parameter is trait bound, to enable testing the method
    pub fn new<T: Iterator<Item=String>>(mut args: T) -> Result<Config, &'static str> {
        args.next(); // get rid of first arg

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Expected a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Expected a filename"),
        };

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
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    // Config::new iterates over std::env::Args object,
    // and Args does not have a public constructor.
    // Hence, to test Config::new, we need to create
    // a collection of "arguments" of our own, which
    // satisfies Iterator trait for Item type String.
    #[derive(Debug)]
    struct ArgsCollection {
        args: Vec<String>,
        index: usize,
    }

    impl Iterator for ArgsCollection {
        type Item = String;
        fn next(&mut self) -> Option<String> {
            if self.index >= self.args.len() {
                return None;
            }
            let result = self.args[self.index].clone();
            self.index += 1;
            Some(result)
        }
    }

    #[test]
    fn test_config_new() {
        let args = ArgsCollection {
            args: vec![String::from("Ignore first arg")],
            index: 0,
        };
        let config = Config::new(args);
        assert!(config.is_err());

        let args = ArgsCollection {
            args: vec![String::from("Still ignored..."),
                       String::from("oh hai"),
            ],
            index: 0,
        };
        let config = Config::new(args);
        assert!(config.is_err());

        let args = ArgsCollection {
            args: vec![String::from("Hello"),
                       String::from("iz lolcat"),
                       String::from("lolfilez.txt"),
            ],
            index: 0,
        };
        let config = Config::new(args);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.query, "iz lolcat");
        assert_eq!(config.filename, "lolfilez.txt");
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
