use std::io;

fn main() {
    println!("**** Celsius to Fahrenheit Temperature Converter ****");
    
    loop {
        println!("Please input a temperature in Celsius:");
        let mut celsius = String::new();

        io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read line");

        let celsius: f32 = match celsius.trim().parse() {
            Ok(f32) => f32,
            Err(_) => continue,
        };

        let fahrenheit: f32 = (celsius * 9.0 / 5.0) + 32.0;

        println!("{:.1} degrees Celsius is {:.1} degrees Fahrenheit", celsius, fahrenheit);
        break;
    }  
}