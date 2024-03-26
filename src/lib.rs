use std::process::exit;

use clap::Parser;

mod cli;

pub fn run() {
    let cli = cli::Cli::parse();
    
    if let Some(value) = cli.value() {
        let result = cli.command().convert(value);
        println!("Result: {result}");
    } else {
        println!("Missing value");
        exit(1);
    }
}
