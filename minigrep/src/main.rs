use std::env;
use std::str::*;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    let config = config.unwrap_or_else(|err| {
        println!("Error reading arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<Error>> { // why Box?
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = String::from_str(&args[1]).unwrap(); // another option would be clone
        let filename = String::from_str(&args[2]).unwrap();
        println!("Searching for {}", query);
        println!("In file {}", filename);
        Ok(Config { query, filename })
    }
}
