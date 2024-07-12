use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, ValueEnum, Debug, Serialize, Deserialize)]
pub enum Temperature {
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
