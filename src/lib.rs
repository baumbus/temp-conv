use clap::Parser;

pub mod cli;
pub mod formatted_output;
pub mod temperature;

pub fn run() {
    let cli = cli::Cli::parse();

    #[cfg(any(feature = "json", feature = "yaml", feature = "toml"))]
    let temperature_in = cli.temperature_in();
    #[cfg(any(feature = "json", feature = "yaml", feature = "toml"))]
    let temperature_out = cli.temperature_out();
    #[cfg(any(feature = "json", feature = "yaml", feature = "toml"))]
    let original_value = cli.value();

    let format = cli.format();

    let result = cli.convert();

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
