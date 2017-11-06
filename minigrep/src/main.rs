extern crate minigrep;

fn main() {
    let config = minigrep::Config::new(std::env::args()).unwrap_or_else(|err| {
        println!("Error reading arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        std::process::exit(1);
    };
}


