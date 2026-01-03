use std::{error::Error, fs::File};

use crate::sensor::{Sensor, SensorType};

#[derive(Debug, serde::Deserialize)]
pub struct DataLine {
    pub sensor_type: String,
    pub value: f64,
    pub timestamp: i32,
}

fn read_data_lines(file: File) -> Result<Vec<Sensor>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(file);
    let mut sensors = Vec::new();

    for result in rdr.deserialize() {
        let record: DataLine = result?;

        match Sensor::try_from(record) {
            Ok(sensor) => sensors.push(sensor),
            Err(e) => eprintln!("Skipping invalid record: {}", e),
        };
    }

    Ok(sensors)
}

