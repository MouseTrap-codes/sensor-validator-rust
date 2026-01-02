use std::f32::consts::E;

fn main() {
    let test_temperature: f64 = 89.6;
    match validate_temperature(test_temperature) {
        Ok(()) => println!("Valid temperature"),
        Err(e) => println!("{}", e),
    }
}

fn validate_temperature(temperature: f64) -> Result<(), &'static str> {
    if temperature < 0.0 {
        Err("INVALID TEMPERATURE: TOO LOW")
    } else if temperature > 100.0 {
        Err("INVALID TEMPERATURE: TOO HIGH")
    } else {
        Ok(())
    }
}

fn validate_pressure(pressure: f64) -> Result<(), &'static str> {
    if pressure < 900.0 {
        Err("INVALID PRESSURE: TOO LOW")
    } else if pressure > 1100.0 {
        Err("INVALID PRESSURE: TOO HIGH")
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // validate_temperature tests
    #[test]
    fn test_valid_temperature() {
        let result = validate_temperature(50.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_temperature_1() {
        let result = validate_temperature(-0.05);
        assert!(result.is_err());
    }
    #[test]
    fn test_invalid_temperature_2() {
        let result = validate_temperature(100.05);
        assert!(result.is_err());
    }

    #[test]
    fn test_low_temperature_message() {
        let err = validate_temperature(-100.0).unwrap_err();
        assert_eq!(err, "INVALID TEMPERATURE: TOO LOW");
    }

      #[test]
    fn test_high_temperature_message() {
        let err = validate_temperature(200.0).unwrap_err();
        assert_eq!(err, "INVALID TEMPERATURE: TOO HIGH");
    }

    // validate_pressure tests

    fn test_valid_pressure() {
        let result = validate_pressure(50.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_pressure_1() {
        let result = validate_pressure(-0.05);
        assert!(result.is_err());
    }
    #[test]
    fn test_invalid_pressure_2() {
        let result = validate_pressure(100.05);
        assert!(result.is_err());
    }

    #[test]
    fn test_low_pressure_message() {
        let err = validate_pressure(-100.0).unwrap_err();
        assert_eq!(err, "INVALID PRESSURE: TOO LOW");
    }

    #[test]
    fn test_high_pressure_message() {
        let err = validate_pressure(1200.0).unwrap_err();
        assert_eq!(err, "INVALID PRESSURE: TOO HIGH");
    }

    // boundary conditions
    #[test]
    fn test_lower_boundary() {
        let res_temp = validate_temperature(0.0);
        let res_press = validate_pressure(900.0);
        assert!(res_temp.is_ok() && res_press.is_ok());
    }

    #[test]
    fn test_higher_boundary() {
        let res_temp = validate_temperature(100.0);
        let res_press = validate_pressure(1100.0);
        assert!(res_temp.is_ok() && res_press.is_ok());
    }


}