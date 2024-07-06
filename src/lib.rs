use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    only_value: bool,
    #[arg(value_enum)]
    temperature_in: Temperature,
    #[arg(value_enum)]
    temperature_out: Temperature,
    value: Option<f64>,
    #[arg(short, long)]
    verbose: bool,
}

impl Cli {
    pub fn only_value(&self) -> bool {
        self.only_value
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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Temperature {
    Celsius,
    Fahrenheit,
    Kelvin,
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

    let result = cli.convert();

    if cli.only_value() {
        println!("{result}");
    } else if cli.verbose() {
        let temp_in = cli.temperature_in();
        let temp_out = cli.temperature_out();
        let original_value = cli.value();
        println!("From {original_value} {temp_in} to {result} {temp_out}");
    } else {
        println!("Result: {result}")
    }
}
