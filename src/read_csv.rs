use crate::sensor::{Sensor, SensorType};

#[derive(Debug, serde::Deserialize)]
pub struct DataLine {
    pub sensor_type: String,
    pub value: f64,
    pub timestamp: i32,
}

