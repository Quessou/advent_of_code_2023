mod calibration_value;
mod read_file;

use calibration_value::CalibrationValue;

use std::path::Path;

fn main() {
    let lines = read_file::read_to_lines(&Path::new("./input_part1.txt"));
    let calibration_values: Vec<CalibrationValue> = lines
        .iter()
        .map(|l| &l as &str)
        .map(|l| CalibrationValue::from(l))
        .collect();
    let gngn = calibration_values.iter();
    let total_value = calibration_values.iter().fold(0, |sum, i| sum + i.0);
    println!("{}", total_value);
}
