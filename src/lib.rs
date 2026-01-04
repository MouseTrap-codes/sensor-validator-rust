pub mod read_csv;
pub mod sensor;

use std::{error::Error, fs::File};

pub use read_csv::{ValidationError, ValidationResult, read_data_lines};
pub use sensor::{Sensor, SensorType};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(&config.file_path)?;

    let results = read_csv::read_data_lines(file)?;

    println!("{}", results.summary());
    println!("Success Rate: {:.2}%", results.success_rate());

    if !results.errors.is_empty() {
        println!("\n--- Error Log ---");
        for err in results.errors {
            println!("Line {}: {}", err.line_number, err.error_message);
        }
    }

    Ok(())
}

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Usage: cargo run <file_path>");
        }

        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}
