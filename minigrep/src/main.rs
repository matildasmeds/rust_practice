use std::env;
use std::str::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = String::from_str(&args[1]).unwrap(); // another option would be clone
        let filename = String::from_str(&args[2]).unwrap();
        println!("Searching for {}", query);
        println!("In file {}", filename);
        Config { query, filename }
    }
}
