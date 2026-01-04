# Sensor Validator 🦀

CLI tool that validates sensor data from CSV files. Checks temperature and pressure readings against valid ranges and reports all errors with line numbers.

## Rules

* **Temperature:** 0-100°C
* **Pressure:** 900-1100 hPa
* **Invalid data:** Collected and logged with line numbers instead of crashing on first error

## Prerequisites

Requires [Rust toolchain](https://rustup.rs/) (1.70+)

## Quick Start
```bash
# validate a CSV file
cargo run test/perfect.csv

# run all tests
cargo test

# generate docs
cargo doc --open
```

## CSV Format
```csv
sensor_type,value,timestamp
temperature,25.5,1000
pressure,1013.25,1001
```

## Usage

**As a CLI tool:**
```bash
cargo run -- path/to/your_data.csv
```

**As a library:**
```rust
use sensor_validator_rust::{read_data_lines, Config, run};
use std::fs::File;

// option 1: use the high-level run() function
let config = Config {
    file_path: String::from("data.csv"),
};
run(config)?;

// option 2: use read_data_lines() directly
let file = File::open("data.csv")?;
let result = read_data_lines(file)?;
println!("Valid: {}", result.valid_sensors.len());
println!("Invalid: {}", result.errors.len());
```

## Example Output
```
Validation Complete:
 Valid: 5
 Invalid: 3
 Total: 8
Success Rate: 62.50%

--- Error Log ---
Line 4: INVALID TEMPERATURE: TOO HIGH
Line 6: INVALID PRESSURE: TOO LOW
Line 7: Unknown sensor type: accelerometer
```

## Design Decisions

**Lib vs Bin:** Core logic is in `lib.rs` with a `run()` function. CLI in `main.rs` only handles argument parsing. You can import the validator into other projects without the CLI code.

**Error collection:** Processes the entire CSV and collects ALL errors instead of failing fast. This way you get the complete picture for debugging instead of fixing one error at a time.

**Config pattern:** Uses a `Config` struct for the file path, making it easy to test the `run()` function without hitting the filesystem.

**Tests:** 
- Unit tests in `sensor.rs` and `read_csv.rs` for validation rules
- Integration tests in `tests/` that run against actual CSV files
- Total: 15+ tests covering boundaries, malformed CSVs, unknown sensor types

## Project Layout
```
src/
├── lib.rs        # Public API + run() function
├── main.rs       # CLI entry point
├── sensor.rs     # Sensor enums and validation logic
└── read_csv.rs   # CSV parsing and error collection

tests/
├── integration_test.rs
test/
├── perfect.csv
├── broken_structure.csv
├── out_of_bounds.csv
└── edge_cases.csv
```

