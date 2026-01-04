use sensor_validator_rust::{Config, read_data_lines, run};
use std::fs::File;

#[test]
fn test_perfect_csv_file() {
    // Path relative to your project root
    let path = "test/perfect.csv";
    let file = File::open(path).expect("perfect.csv missing from test folder");

    let result = read_data_lines(file).unwrap();

    // perfect.csv should have 4 valid sensors and 0 errors
    assert_eq!(result.valid_sensors.len(), 4);
    assert_eq!(result.errors.len(), 0);
    assert_eq!(result.success_rate(), 100.0);
}

#[test]
fn test_edge_cases_file() {
    let path = "test/edge_cases.csv";
    let file = File::open(path).expect("edge_cases.csv missing");

    let result = read_data_lines(file).unwrap();

    // Based on our previous run: 0 valid, 4 errors
    assert_eq!(result.valid_sensors.len(), 0);
    assert_eq!(result.errors.len(), 4);
}

#[test]
fn test_run_function_executes() {
    // This tests the 'run' function in lib.rs directly
    let config = Config {
        file_path: String::from("test/perfect.csv"),
    };

    // run() returns Result<(), Box<dyn Error>>
    let result = run(config);

    assert!(
        result.is_ok(),
        "The run function should execute without error"
    );
}
