use std::process::exit;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_enum)]
    temperature_in: Temperature,
    #[arg(value_enum)]
    temperature_out: Temperature,
    value: Option<f64>,
}

impl Cli {
    pub fn convert(&self) -> f64 {
        let temp = self.value.expect("No value given");
        if self.temperature_in == self.temperature_out {
            return temp;
        };

        match self.temperature_in {
            Temperature::Celsius => match self.temperature_out {
                Temperature::Celsius => unreachable!(),
                Temperature::Kelvin => temp + 273.15,
                Temperature::Fahrenheit => temp * 1.8 + 32.0,
            },
            Temperature::Kelvin => match self.temperature_out {
                Temperature::Celsius => temp - 273.15,
                Temperature::Kelvin => unreachable!(),
                Temperature::Fahrenheit => (temp - 273.15) * 1.8 + 32.0,
            },
            Temperature::Fahrenheit => match self.temperature_out {
                Temperature::Celsius => (temp - 32.0) * 1.8,
                Temperature::Kelvin => (temp - 32.0) * 1.8 + 273.15,
                Temperature::Fahrenheit => unreachable!(),
            },
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Temperature {
    Celsius,
    Kelvin,
    Fahrenheit,
}

pub fn run() {
    let cli = Cli::parse();

    let result = cli.convert();
    println!("Result: {result}");
}
