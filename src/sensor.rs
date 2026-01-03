use crate::read_csv::DataLine;

#[derive(Debug, PartialEq)]
pub enum SensorType {
    Temperature(f64),
    Pressure(f64),
}

#[derive(Debug)]
pub struct Sensor {
    sensor_data: SensorType,
    timestamp: u64,
}

impl Sensor {
    fn validate(&self) -> Result<(), &'static str> {
        match &self.sensor_data {
            SensorType::Temperature(temperature) => {
                if *temperature < 0.0 {
                    Err("INVALID TEMPERATURE: TOO LOW")
                } else if *temperature > 100.0 {
                    Err("INVALID TEMPERATURE: TOO HIGH")
                } else {
                    Ok(())
                }
            }

            SensorType::Pressure(pressure) => {
                if *pressure < 900.0 {
                    Err("INVALID PRESSURE: TOO LOW")
                } else if *pressure > 1100.0 {
                    Err("INVALID PRESSURE: TOO HIGH")
                } else {
                    Ok(())
                }
            }
            
        }
    }  
}

impl TryFrom<DataLine> for Sensor {
    type Error = String;

    fn try_from(line: DataLine) -> Result<Self, Self::Error> {
        let sensor_data = match line.sensor_type.to_lowercase().as_str() {
            "temperature" => SensorType::Temperature(line.value),
            "pressure" => SensorType::Pressure(line.value),
            _ => return Err(format!("Unknown sensor type: {}", line.sensor_type))
        };

        let sensor = Sensor {
            sensor_data,
            timestamp: line.timestamp as u64
        };

        sensor.validate().map_err(|e| e.to_string())?;

        Ok(sensor)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // temperature validation tests
    #[test]
    fn temperature_valid() {
        let sensor = Sensor {
            sensor_data: SensorType::Temperature(50.0),
            timestamp: 10000,
        };

        assert!(sensor.validate().is_ok());
    }

    #[test]
    fn temperature_low() {
        let sensor = Sensor {
            sensor_data: SensorType::Temperature(-1.0),
            timestamp: 10000,
        };

        assert_eq!(sensor.validate().unwrap_err(), "INVALID TEMPERATURE: TOO LOW");
    }

    #[test]
    fn temperature_high() {
        let sensor = Sensor {
            sensor_data: SensorType::Temperature(150.0),
            timestamp: 10000,
        };

        assert_eq!(sensor.validate().unwrap_err(), "INVALID TEMPERATURE: TOO HIGH");
    }

    // pressure validation tests
    #[test]
    fn pressure_valid() {
        let sensor = Sensor {
            sensor_data: SensorType::Pressure(950.0),
            timestamp: 10000,
        };

        assert!(sensor.validate().is_ok());
    }

    #[test]
    fn pressure_low() {
        let sensor = Sensor {
            sensor_data: SensorType::Pressure(850.0),
            timestamp: 10000,
        };

        assert_eq!(sensor.validate().unwrap_err(), "INVALID PRESSURE: TOO LOW");
    }

    #[test]
    fn pressure_high() {
        let sensor = Sensor {
            sensor_data: SensorType::Pressure(1150.0),
            timestamp: 10000,
        };

        assert_eq!(sensor.validate().unwrap_err(), "INVALID PRESSURE: TOO HIGH");
    }

    // test boundaries
    #[test]
    fn test_boundaries() {
        let sensor_temp_lower = Sensor {
            sensor_data: SensorType::Temperature(0.0),
            timestamp: 10000,
        };

        let sensor_temp_higher = Sensor {
            sensor_data: SensorType::Temperature(100.0),
            timestamp: 10000,
        };

        let sensor_pressure_lower = Sensor {
            sensor_data: SensorType::Pressure(900.0),
            timestamp: 10000,
        };

        let sensor_pressure_higher = Sensor {
            sensor_data: SensorType::Pressure(1100.0),
            timestamp: 10000,
        };

        assert!(sensor_temp_lower.validate().is_ok());
        assert!(sensor_temp_higher.validate().is_ok());
        assert!(sensor_pressure_lower.validate().is_ok());
        assert!(sensor_pressure_lower.validate().is_ok());
    }

    // test tryFrom
    #[test]
    fn test_tryFrom_success() {
        let line = DataLine {
            sensor_type: String::from("temperature"),
            value: 50.0,
            timestamp: 000
        };

        let sensor = Sensor::try_from(line).unwrap();

        let expected = Sensor {
            sensor_data: SensorType::Temperature(50.0),
            timestamp: 000,
        };

        assert_eq!(sensor.sensor_data, expected.sensor_data);
    }

     #[test]
    fn test_tryFrom_failure() {
        let line = DataLine {
            sensor_type: String::from("nonchalant"),
            value: 50.0,
            timestamp: 000
        };

        let err = Sensor::try_from(line).unwrap_err();

        assert_eq!(err, "Unknown sensor type: nonchalant");
    }




}