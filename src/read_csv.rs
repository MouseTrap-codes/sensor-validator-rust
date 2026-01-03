use std::{error::Error, io::Read};

use crate::sensor::{Sensor, SensorType};

#[derive(Debug, serde::Deserialize)]
pub struct DataLine {
    pub sensor_type: String,
    pub value: f64,
    pub timestamp: i32,
}

#[derive(Debug)]
pub struct ValidationResult {
    pub valid_sensors: Vec<Sensor>,
    pub errors: Vec<ValidationError>,
}

#[derive(Debug)]
pub struct ValidationError {
    pub line_number: usize,
    pub error_message: String,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            valid_sensors: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "Validation Complete:\n Valid: {}\n Invalid: {}\n Total: {}",
            self.valid_sensors.len(),
            self.errors.len(),
            self.valid_sensors.len() + self.errors.len()
        )
    }

    pub fn success_rate(&self) -> f64 {
        let total = self.valid_sensors.len() + self.errors.len();
        if total == 0 {
            return 0.0
        }

        (self.valid_sensors.len() as f64 / total as f64) * 100.0
    }
}


pub fn read_data_lines<R: Read>(reader: R) -> Result<ValidationResult, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut result = ValidationResult::new();

    for (i, record_result)  in rdr.deserialize::<DataLine>().enumerate() {
        let line_number = i + 2;

        match record_result {
            Ok(record) => {
                match Sensor::try_from(record) {
                    Ok(sensor) => result.valid_sensors.push(sensor),
                    Err(e) => {
                        eprintln!("Line {}: {}", line_number, e);
                        result.errors.push(ValidationError { line_number, error_message: e });
                    }
                }
            }
            Err(e) => {
                let err_msg = format!("CSV parse error: {}", e);
                eprintln!("{}: {}", line_number, err_msg);
                result.errors.push(ValidationError { line_number, error_message: err_msg });
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod csv_tests {
    use super::*;

    #[test]
    fn test_read_all_valid() {
        let data = "sensor_type,value,timestamp
temperature,25.0,1000
pressure,1013.0,2000";

        let result = read_data_lines(data.as_bytes()).unwrap();

        assert_eq!(result.valid_sensors.len(), 2);
        assert_eq!(result.errors.len(), 0);
        assert_eq!(result.success_rate(), 100.0);
    }

    #[test]
    fn test_mixed_data_quality() {
        // Line 2: Valid
        // Line 3: Invalid Value (Temp too high)
        // Line 4: Invalid Format (Missing columns)
        // Line 5: Invalid Type (Unknown sensor)
        let data = "sensor_type,value,timestamp
temperature,25.0,1000
temperature,500.0,2000
pressure,950.0
unknown,10.0,4000";

        let result = read_data_lines(data.as_bytes()).unwrap();

        assert_eq!(result.valid_sensors.len(), 1, "Should only have 1 valid sensor");
        assert_eq!(result.errors.len(), 3, "Should have 3 errors recorded");
        
        // verify line numbers are accurate
        assert_eq!(result.errors[0].line_number, 3); // logic error
        assert_eq!(result.errors[1].line_number, 4); // parse error
        assert_eq!(result.errors[2].line_number, 5); // type error
    }

    #[test]
    fn test_empty_csv() {
        let data = "sensor_type,value,timestamp";
        let result = read_data_lines(data.as_bytes()).unwrap();

        assert_eq!(result.valid_sensors.len(), 0);
        assert_eq!(result.success_rate(), 0.0);
    }

    #[test]
    fn test_malformed_header() {
        // serde will fail here because column names don't match
        let data = "type,val,time
temperature,25.0,1000";
        
        let result = read_data_lines(data.as_bytes()).unwrap();
        assert!(result.errors.len() > 0);
        assert_eq!(result.valid_sensors.len(), 0);
    }
}