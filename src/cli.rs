use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[cfg(any(feature = "json", feature = "yaml", feature = "toml"))]
    #[arg(value_enum, short, long)]
    format: Option<crate::formatted_output::Format>,
    #[arg(long)]
    only_value: bool,
    #[arg(short, long)]
    output: Option<PathBuf>,
    #[arg(value_enum)]
    temperature_in: crate::temperature::Temperature,
    #[arg(value_enum)]
    temperature_out: crate::temperature::Temperature,
    value: Option<f64>,
    #[arg(short, long)]
    verbose: bool,
}

impl Cli {
    #[cfg(any(feature = "json", feature = "yaml", feature = "toml"))]
    pub fn format(&self) -> Option<crate::formatted_output::Format> {
        self.format
    }

    #[cfg(not(any(feature = "json", feature = "yaml", feature = "toml")))]
    pub fn format(&self) -> Option<crate::formatted_output::Format> {
        None
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

    pub fn temperature_in(&self) -> crate::temperature::Temperature {
        self.temperature_in
    }

    pub fn temperature_out(&self) -> crate::temperature::Temperature {
        self.temperature_out
    }

    pub fn convert(&self) -> anyhow::Result<f64> {
        use crate::temperature::Temperature;

        let temp = self
            .value
            .context("Failed to convert the temperature because there was no temperature given.")?;
        if self.temperature_in == self.temperature_out {
            return Ok(temp);
        };

        let temp = match self.temperature_in {
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
        };

        Ok(temp)
    }
}
