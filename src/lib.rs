use std::{env, process::exit};

pub enum Temperature {
    Fahrenheit(f64),
    Celsius(f64),
    Kelvin(f64),
}

impl Temperature {
    pub fn to_fahrenheit(&self) -> f64 {
        match self {
            Temperature::Celsius(c) => *c * 1.8 + 32.0,
            Temperature::Fahrenheit(f) => *f,
            Temperature::Kelvin(k) => (*k - 273.15) * 1.8 + 32.0,
        }
    }

    pub fn to_celsius(&self) -> f64 {
        match self {
            Temperature::Celsius(c) => *c,
            Temperature::Fahrenheit(f) => (*f - 32.0) * (1.0 / 1.8),
            Temperature::Kelvin(k) => *k - 273.15,
        }
    }

    pub fn to_kelvin(&self) -> f64 {
        match self {
            Temperature::Celsius(c) => *c + 273.15,
            Temperature::Fahrenheit(f) => (*f - 32.0) * (1.0 / (1.8 + 273.15)),
            Temperature::Kelvin(k) => *k,
        }
    }
}

pub enum Mode {
    FahrenheitToCelsius(Temperature),
    FahrenheitToKelvin(Temperature),
    CelsiusToFahrenheit(Temperature),
    CelsiusToKelvin(Temperature),
    KelvinToCelsius(Temperature),
    KelvinToFahrenheit(Temperature),
}

impl Mode {
    pub fn execute(&self) -> f64 {
        match self {
            Mode::FahrenheitToCelsius(f) => f.to_celsius(),
            Mode::FahrenheitToKelvin(f) => f.to_kelvin(),
            Mode::CelsiusToFahrenheit(c) => c.to_fahrenheit(),
            Mode::CelsiusToKelvin(c) => c.to_kelvin(),
            Mode::KelvinToCelsius(k) => k.to_celsius(),
            Mode::KelvinToFahrenheit(k) => k.to_fahrenheit(),
        }
    }
}

pub fn run() {
    let mode = parse_arguments();

    println!("{}", mode.execute())
}

fn display_help() {
    println!("Temperature converter\n");

    println!("Usage: temp-conv [TEMPERATURE] [CONVERSION]\n");

    println!("Conversions: ");

    println!("  ctf");
    println!("  ctk");
    println!("  ftc");
    println!("  ftk");
    println!("  ktc");
    println!("  ktf");
}

fn parse_arguments() -> Mode {
    let args: Vec<String> = env::args().collect();

    let modus = args.get(2).unwrap_or(&String::from("ctf")).clone();
    let temp_string = args
        .get(1)
        .unwrap_or_else(|| {
            display_help();
            exit(0);
        })
        .clone();
    let temp: f64;

    if temp_string == "--help" {
        display_help();
        exit(0);
    } else {
        temp = temp_string
            .parse::<f64>()
            .expect("Expected a valid temperature");
    }
    match modus.as_str() {
        "ctf" => Mode::CelsiusToFahrenheit(Temperature::Celsius(temp)),
        "ctk" => Mode::CelsiusToKelvin(Temperature::Celsius(temp)),
        "ftc" => Mode::FahrenheitToCelsius(Temperature::Fahrenheit(temp)),
        "ftk" => Mode::FahrenheitToKelvin(Temperature::Fahrenheit(temp)),
        "ktc" => Mode::KelvinToCelsius(Temperature::Kelvin(temp)),
        "ktf" => Mode::KelvinToFahrenheit(Temperature::Kelvin(temp)),
        _ => Mode::CelsiusToFahrenheit(Temperature::Celsius(temp)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn celsius_to_fahrenheit() {
        let temp = Temperature::Celsius(0.0);

        assert_eq!(temp.to_fahrenheit(), 32.0)
    }

    #[test]
    fn celsius_to_kelvin() {
        let temp = Temperature::Celsius(0.0);

        assert_eq!(temp.to_kelvin(), 273.15)
    }

    #[test]
    fn fahrenheit_to_celsius() {
        let temp = Temperature::Fahrenheit(0.0);

        assert_eq!(temp.to_celsius(), -17.77777777777778)
    }

    #[test]
    fn fahrenheit_to_kelvin() {
        let temp = Temperature::Fahrenheit(0.0);

        assert_eq!(temp.to_kelvin(), -0.11638479723586106)
    }

    #[test]
    fn kelvin_to_fahrenheit() {
        let temp = Temperature::Kelvin(0.0);

        assert_eq!(temp.to_fahrenheit(), -459.66999999999996)
    }

    #[test]
    fn kelvin_to_celsius() {
        let temp = Temperature::Kelvin(0.0);

        assert_eq!(temp.to_celsius(), -273.15)
    }
}
