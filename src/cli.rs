//! Struct and helper function for the cli tool

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
    /// Returns the format of this [`Cli`].
    ///
    /// If none of the features json, yaml or toml is set this function will always return None.
    #[cfg(any(feature = "json", feature = "yaml", feature = "toml"))]
    #[inline]
    pub const fn format(&self) -> Option<crate::formatted_output::Format> {
        self.format
    }

    /// Returns the format of this [`Cli`].
    ///
    /// If none of the features json, yaml or toml is set this function will always return None.
    #[cfg(not(any(feature = "json", feature = "yaml", feature = "toml")))]
    #[inline]
    pub const fn format(&self) -> Option<crate::formatted_output::Format> {
        None
    }

    /// Returns the only value of this [`Cli`].
    #[inline]
    pub const fn only_value(&self) -> bool {
        self.only_value
    }

    /// Returns the output of this [`Cli`].
    #[inline]
    pub fn output(&self) -> Option<PathBuf> {
        self.output.clone()
    }

    /// Returns the value of this [`Cli`].
    ///
    /// # Panics
    ///
    /// Panics if value is none.
    #[inline]
    pub fn value(&self) -> f64 {
        self.value.expect("Value missing")
    }

    /// Returns the verbose of this [`Cli`].
    #[inline]
    pub const fn verbose(&self) -> bool {
        self.verbose
    }

    /// Returns the temperature in of this [`Cli`].
    #[inline]
    pub const fn temperature_in(&self) -> crate::temperature::Temperature {
        self.temperature_in
    }

    /// Returns the temperature out of this [`Cli`].
    #[inline]
    pub const fn temperature_out(&self) -> crate::temperature::Temperature {
        self.temperature_out
    }

    /// Returns the convert of this [`Cli`].
    ///
    /// # Errors
    ///
    /// This function will return an error if the field value of this struct is none.
    #[inline]
    pub fn convert(&self) -> anyhow::Result<f64> {
        let temp = self
            .value
            .context("Failed to convert the temperature because there was no temperature given.")?;

        if self.temperature_in == self.temperature_out {
            return Ok(temp);
        };

        Ok(self.temperature_in.convert(self.temperature_out, temp))
    }
}
