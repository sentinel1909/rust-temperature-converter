use clap::Parser;

#[derive(Debug)]
struct Temperature(f32);

#[derive(Parser, Debug)]
#[clap(name = "Rust Temperature Converter")]
#[clap(author = "Jeff Mitchell <sentinel1909@protonmail.com>")]
#[clap(version = "0.1.0")]
#[clap(about = "Converts a Celsius temperature into Fahrenheit and Kelvin", long_about = None)]
#[clap(allow_negative_numbers = true)]
struct Args {
    #[clap(short, long, value_parser)]
    temp: f32
}

impl Temperature {
    fn new(input_temp: f32) -> Self {
        Self(input_temp)
    }

    fn to_fahrenheit(&self) -> Self {
        Self(self.0 * (9.0 / 5.0) + 32.0)
    }

    fn to_kelvin(&self) -> Self {
        Self(self.0 + 273.15)
    }
}

fn main() {
    let args = Args::parse();
    let celsius_temp = Temperature::new(args.temp);
    let fahrenheit_temp = Temperature::to_fahrenheit(&celsius_temp);
    let kelvin_temp = Temperature::to_kelvin(&celsius_temp);
    println!("Converting {} Celsius into Fahrenheit and Kelvin...", args.temp);
    println!("The Celsius temperature is: {:?}", celsius_temp.0);
    println!("The Fahrenheit temperature is: {:?}", fahrenheit_temp.0);
    println!("The Kelvin temperature is: {:?}", kelvin_temp.0);
}
