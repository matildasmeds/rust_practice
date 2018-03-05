// Inspired by https://wunder.dog/hassuimmat-sanat
// The task is to find the funniest words in a given text,
// as ranked by their "funniness score".
#![feature(test)]
extern crate test;
extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read_file(path: &str) -> String {
    let path = Path::new(&path);
    let mut file = match File::open(&path) {
        Err(err) => panic!("Could not open {}: {}", path.display(),
                                                   err.description()),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(err) => panic!("Could not read file: {}", err.description()),
        Ok(_) => s
    }
}

fn collect_scores(text: &str) -> HashMap<usize, Vec<&str>> {
    let re = regex();
    let mut scores = HashMap::new();
    for word in text.split_whitespace() {
        let funny_score = funny_score(word, &re);
        scores
            .entry(funny_score)
            .or_insert(vec!())
            .push(word);
    }
    scores
}

fn funny_score(word: &str, re: &Regex) -> usize {
    let mut score = 0;
    for cap in re.captures_iter(word) {
        for i in 0..cap.len() {
            let count = cap[i].chars().count();
            score += count << count;
        }
    }
    score
}

fn regex() -> Regex {
    Regex::new(r"[aeiouyäö]+").unwrap()
}

fn main() {
    let text = read_file("lib/alastalon_salissa.txt");
    let scores = collect_scores(&text);
    let max_score = scores.keys().max().unwrap();
    println!("Max score: {}", max_score);
    for word in scores.get(max_score).unwrap() {
        println!("{}", word);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self::test::Bencher;

    // Creating a Regex in Rust is expensive, but using it is not.
    // Run cargo bench to see.
    #[bench]
    fn bench_regex(b: &mut Bencher) {
        b.iter(|| { regex(); });
    }

    #[bench]
    fn bench_funny_score(b: &mut Bencher) {
        let re = regex();
        b.iter(|| { funny_score("maakuntauudistus", &re); });
    }

    #[test]
    fn test_funny_score() {
        let re = regex();
        assert_eq!(funny_score("koira", &re), 10);
        assert_eq!(funny_score("hääyöaie", &re), 896);
    }
}
