use std::io;

fn main() {
   loop {
        println!("[1] Convert Fahrenheit->Celsius");
        println!("[2] Convert Celsius->Fahrenheit");
        println!("[3] Quit");

        let mut mode = String::new();
        let converter: fn(f64) -> f64;
        let mut degrees = String::new();
        let mut units = ["Fahrenheit", "Celsius"];

        io::stdin().read_line(&mut mode)
            .expect("Failed to read line");

        match mode.trim() {
            "1" => { converter = fahrenheit_to_celcius; },
            "2" => { converter = celcius_to_fahrenheit; units.reverse() },
            "3" => break,
            _ => continue,
        }

        println!("Enter degrees to be converted in {}:", units[0]);
        io::stdin().read_line(&mut degrees)
            .expect("Failed to read line");
        let mut degrees: f64 = degrees.trim().parse()
            .expect("Please type a number!");

        degrees = converter(degrees);
        let result = format!("{:.*}", 1, degrees);
        println!("That makes {} degrees in {}!", result, units[1]);
    }
}

fn fahrenheit_to_celcius(x: f64) -> f64 {
    (x - 32.0) * (5.0 / 9.0)
}

fn celcius_to_fahrenheit(x: f64) -> f64 {
    x * ( 5.0 / 9.0) + 32.02
}
