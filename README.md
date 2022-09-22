# Simple Rust Temperature Converter

This is a simple Rust program which uses the clap crate to receive a temperature in Celsius from a command line argument. It then converts the Celsius temperature into Fahrenheit and Kelvin, then outputs the converted values to the console.

The input temperature must be a floating point number, i.e. 25.0.

You can enter negative temperature values, i.e. -25.0.

## Usage

cargo run -- --temp <celsius_temp>
