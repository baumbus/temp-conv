#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::missing_inline_in_public_items)]

//! # temp-conv
//!
//! temp-conv is a framework for converting between different temperature units, but it has also it's own cli-tool which uses said framework.

use anyhow::Result;
use clap::Parser;

pub mod cli;
pub mod error;
pub mod formatted_output;
pub mod temperature;

/// Runs the main logic of this tool
///
/// # Panics
///
/// Panics if the output can not be parsed to the specified format.
///
/// # Errors
///
/// This function will return an error if the conversion is resulting in an I/O error occurs.
#[inline]
pub fn run() -> Result<()> {
    let cli = cli::Cli::parse();

    #[cfg(any(feature = "json", feature = "yaml", feature = "toml"))]
    let temperature_in = cli.temperature_in();
    #[cfg(any(feature = "json", feature = "yaml", feature = "toml"))]
    let temperature_out = cli.temperature_out();
    #[cfg(any(feature = "json", feature = "yaml", feature = "toml"))]
    let original_value = cli.value();

    let format = cli.format();

    let result = cli.convert()?;

    #[cfg(any(feature = "json", feature = "yaml", feature = "toml"))]
    let output =
        formatted_output::Output::new(temperature_in, original_value, temperature_out, result);

    let output_string = if cli.only_value() {
        format!("{result}")
    } else if let Some(format) = format {
        match format {
            #[cfg(feature = "json")]
            formatted_output::Format::Json => output.to_json().unwrap(),
            #[cfg(feature = "yaml")]
            formatted_output::Format::Yaml => output.to_yaml().unwrap(),
            #[cfg(feature = "toml")]
            formatted_output::Format::Toml => output.to_toml().unwrap(),
        }
    } else {
        format!("Result: {result}")
    };

    if let Some(path) = cli.output() {
        std::fs::write(path.as_path(), output_string)?;

        if cli.verbose() {
            println!("Sucessfully saved at {}", path.display());
        }
    } else {
        println!("{}", output_string);
    };

    Ok(())
}
