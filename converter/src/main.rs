use std::fmt;
use std::io;

/* Further ideas
  * Handle errors properly
  * Mode checked twice, make it DRY
  * Temperature / Conversions Module
  * Handle Kelvin degrees
*/

enum Temperature<F64> {
    Celsius(F64),
    Fahrenheit(F64),
}

impl fmt::Display for Temperature<f64> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Temperature::Celsius(x) => write!(f, "{:.*}° Celsius", 1, x),
            Temperature::Fahrenheit(x) => write!(f, "{:.*}° Fahrenheit", 1, x),
        }
    }
}

fn convert(temp: Temperature<f64>) -> Temperature<f64> {
    match temp {
        Temperature::Celsius(x) => Temperature::Fahrenheit(x * (9.0 / 5.0) + 32.0),
        Temperature::Fahrenheit(x) => Temperature::Celsius((x - 32.0) * (5.0 / 9.0)),
    }
}


fn main() {
   loop {
        // Print options
        println!("[1] Convert Fahrenheit->Celsius");
        println!("[2] Convert Celsius->Fahrenheit");
        println!("[Q] Quit");

        // Get mode from user
        let mut mode = String::new();

        io::stdin().read_line(&mut mode)
            .expect("Failed to read line");

        // Get degrees from user
        match mode.trim() {
            "1" => println!("Enter degrees in Fahrenheit"),
            "2" => println!("Enter degrees in Celsius"),
            "Q" | "q" => break,
            _ => continue,
        }

        let mut degrees = String::new();
        io::stdin().read_line(&mut degrees)
            .expect("Failed to read line");
        let degrees: f64 = degrees.trim().parse()
            .expect("Please type a number!");

        // Calculate result
        let result;
        match mode.trim() {
            "1" => { result = convert(Temperature::Fahrenheit(degrees)) },
            _ => { result = convert(Temperature::Celsius(degrees)) },
        }

        // Print result
        println!("That makes {}!", result);
    }
}
