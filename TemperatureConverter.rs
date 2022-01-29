use std::io;

fn main() {
    println!("**** Celsius to Fahrenheit Temperature Converter ****");
    println!("Please input a temperature in Celsius:");

    let mut celsius = String::new();

    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read line");

    let celsius: f32 = match celsius.trim().parse() {
        Ok(f32) => f32,
        Err(_error) => panic!("Please enter a number!"),
    };

    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    
    println!("{} degrees Celsius is {} degrees Fahrenheit", celsius, fahrenheit);
}