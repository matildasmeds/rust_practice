use std::fmt;

#[derive(Debug)]
enum Suit {
    Heart,
    Diamond,
    Spade,
    Club
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Suit::Heart => write!(f, "♥"),
           Suit::Diamond => write!(f, "Diamond"),
           Suit::Spade => write!(f, "Spade"),
           Suit::Club => write!(f, "Club"),
       }
    }
}

fn main() {
    let s = Suit::Heart;
    println!("{}", s);
}

