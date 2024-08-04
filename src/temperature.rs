use std::str::FromStr;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

use crate::error::TemperatureParseError;

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, ValueEnum, Debug, Serialize, Deserialize)]
pub enum Temperature {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl Temperature {
    const KELVIN_BASELINE: f64 = 273.15;
    const FAHRENHEIT_COEF: f64 = 5.0 / 9.0;
    const REVERSE_FAHRENHEIT_COEF: f64 = 9.0 / 5.0;
    const FAHRENHEIT_BASELINE: f64 = 32.0;

    pub fn convert(&self, target_temperature: Temperature, value: f64) -> f64 {
        match self {
            Temperature::Celsius => match target_temperature {
                Temperature::Celsius => value,
                Temperature::Kelvin => value + Temperature::KELVIN_BASELINE,
                Temperature::Fahrenheit => {
                    value * Temperature::REVERSE_FAHRENHEIT_COEF + Temperature::FAHRENHEIT_BASELINE
                }
            },
            Temperature::Kelvin => match target_temperature {
                Temperature::Celsius => value - Temperature::KELVIN_BASELINE,
                Temperature::Kelvin => value,
                Temperature::Fahrenheit => {
                    (value - Temperature::KELVIN_BASELINE) * Temperature::REVERSE_FAHRENHEIT_COEF
                        + Temperature::FAHRENHEIT_BASELINE
                }
            },
            Temperature::Fahrenheit => match target_temperature {
                Temperature::Celsius => {
                    (value - Temperature::FAHRENHEIT_BASELINE) * Temperature::FAHRENHEIT_COEF
                }
                Temperature::Kelvin => {
                    (value - Temperature::FAHRENHEIT_BASELINE) * Temperature::FAHRENHEIT_COEF
                        + Temperature::KELVIN_BASELINE
                }
                Temperature::Fahrenheit => value,
            },
        }
    }
}

impl FromStr for Temperature {
    type Err = crate::error::TemperatureParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Kelvin" | "kelvin" | "k" => Ok(Temperature::Kelvin),
            "Celsius" | "celsius" | "c" => Ok(Temperature::Celsius),
            "Fahrenheit" | "fahrenheit" | "f" => Ok(Temperature::Fahrenheit),
            _ => Err(TemperatureParseError::InvalidTemperature(s.to_string())),
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{
        super::error::TemperatureParseError,
        Temperature::{self, *},
    };

    use assert_float_eq::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(Celsius, Kelvin, 0.0, 273.15)]
    #[case(Celsius, Kelvin, 27.0, 300.15)]
    #[case(Celsius, Kelvin, -200.0, 73.15)]
    #[case(Celsius, Kelvin, 9031.99, 9305.14)]
    #[case(Kelvin, Celsius, 273.15, 0.0)]
    #[case(Kelvin, Celsius, 300.15, 27.0)]
    #[case(Kelvin, Celsius, 9031.99, 8758.84)]
    #[case(Kelvin, Celsius, -9031.99, -9305.14)]
    #[case(Fahrenheit, Celsius, 32.0, 0.0)]
    #[case(Fahrenheit, Celsius, 50.0, 10.0)]
    #[case(Fahrenheit, Celsius, 14.0, -10.0)]
    #[case(Fahrenheit, Celsius, -4.0, -20.0)]
    #[case(Celsius, Fahrenheit, 5.0, 41.0)]
    #[case(Celsius, Fahrenheit, -5.0, 23.0)]
    #[case(Celsius, Fahrenheit, 27.0, 80.6)]
    #[case(Celsius, Fahrenheit, 0.0, 32.0)]
    #[case(Fahrenheit, Kelvin, 32.0, 273.15)]
    #[case(Fahrenheit, Kelvin, 50.0, 283.15)]
    #[case(Fahrenheit, Kelvin, 14.0, 263.15)]
    #[case(Fahrenheit, Kelvin, -4.0, 253.15)]
    #[case(Kelvin, Fahrenheit, 0.0, -459.67)]
    #[case(Kelvin, Fahrenheit, 100.0, -279.67)]
    #[case(Kelvin, Fahrenheit, 155.5, -179.77)]
    #[case(Kelvin, Fahrenheit, 2500.0, 4040.33)]
    fn check_conversion(
        #[case] original_temperature: Temperature,
        #[case] target_temperature: Temperature,
        #[case] original_value: f64,
        #[case] expected_value: f64,
    ) {
        let result = original_temperature.convert(target_temperature, original_value);

        assert_f64_near!(result, expected_value);
    }

    #[rstest]
    #[case("Kelvin", Ok(Temperature::Kelvin))]
    #[case("kelvin", Ok(Temperature::Kelvin))]
    #[case("k", Ok(Temperature::Kelvin))]
    #[case("Celsius", Ok(Temperature::Celsius))]
    #[case("celsius", Ok(Temperature::Celsius))]
    #[case("c", Ok(Temperature::Celsius))]
    #[case("Fahrenheit", Ok(Temperature::Fahrenheit))]
    #[case("fahrenheit", Ok(Temperature::Fahrenheit))]
    #[case("f", Ok(Temperature::Fahrenheit))]
    fn check_parsing(
        #[case] input: &str,
        #[case] expected: Result<Temperature, TemperatureParseError>,
    ) {
        assert_eq!(Temperature::from_str(input), expected);
    }
}
