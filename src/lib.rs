use std::process::exit;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    value: Option<f64>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Celsius to Fahrenheit
    Ctf,
    /// Celsius to Kelvin
    Ctk,
    /// Fahrenheit to Celsius
    Ftc,
    /// Fahrenheit to Kelvin
    Ftk,
    // Kelvin to Celsius
    Ktc,
    /// Kelvin to Fahrenheit
    Ktf,
}

impl Commands {
    pub fn convert(&self, value: f64) -> f64 {
        match self {
            Self::Ctf => value * 1.8 + 32.0,
            Self::Ctk => value + 273.15,
            Self::Ftc => (value - 32.0) * 1.8,
            Self::Ftk => (value - 32.0) * 1.8 + 273.15,
            Self::Ktc => value - 273.15,
            Self::Ktf => (value - 273.15) * 1.8 + 32.0, 
        }
    }
}

pub fn run() {
    let cli = Cli::parse();
    
    if let Some(value) = cli.value {
        let result = cli.command.convert(value);
        println!("Result: {result}");
    } else {
        println!("Missing value");
        exit(1);
    }
}
