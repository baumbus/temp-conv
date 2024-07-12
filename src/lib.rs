use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    json: bool,
    #[arg(long)]
    only_value: bool,
    #[arg(short, long)]
    output: Option<PathBuf>,
    #[arg(value_enum)]
    temperature_in: Temperature,
    #[arg(value_enum)]
    temperature_out: Temperature,
    value: Option<f64>,
    #[arg(short, long)]
    verbose: bool,
}

impl Cli {
    pub fn json(&self) -> bool {
        self.json
    }

    pub fn only_value(&self) -> bool {
        self.only_value
    }

    pub fn output(&self) -> Option<PathBuf> {
        self.output.clone()
    }

    pub fn value(&self) -> f64 {
        self.value.expect("Value missing")
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }

    pub fn temperature_in(&self) -> Temperature {
        self.temperature_in
    }

    pub fn temperature_out(&self) -> Temperature {
        self.temperature_out
    }

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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Serialize, Deserialize)]
enum Temperature {
    Celsius,
    Fahrenheit,
    Kelvin,
}

#[derive(Serialize, Deserialize)]
struct Output {
    temperature_in: Temperature,
    temperature_out: Temperature,
    value_in: f64,
    value_out: f64,
}

impl Output {
    /// Creates a new [`Output`].
    fn new(
        temperature_in: Temperature,
        value_in: f64,
        temperature_out: Temperature,
        value_out: f64,
    ) -> Self {
        Self {
            temperature_in,
            value_in,
            temperature_out,
            value_out,
        }
    }
}

impl std::fmt::Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Temperature::Celsius => write!(f, "Celsius"),
            Temperature::Fahrenheit => write!(f, "Fahrenheit"),
            Temperature::Kelvin => write!(f, "Kelvin"),
        }
    }
}

pub fn run() {
    let cli = Cli::parse();

    let temperature_in = cli.temperature_in();
    let temperature_out = cli.temperature_out();
    let original_value = cli.value();

    let result = cli.convert();

    let output = Output::new(temperature_in, original_value, temperature_out, result);

    let output_string = if cli.only_value() {
        format!("{result}")
    } else if cli.json() {
        let json_string = serde_json::to_string_pretty(&output).unwrap();
        json_string.to_string()
    } else {
        format!("Result: {result}")
    };

    if let Some(path) = cli.output() {
        let result = std::fs::write(path.as_path(), output_string);
        match result {
            Ok(_) if cli.verbose() => println!("Sucessfully saved at {}", path.display()),
            Err(e) => eprintln!("{e}"),
            _ => {}
        }
    } else {
        println!("{}", output_string);
    }
}
