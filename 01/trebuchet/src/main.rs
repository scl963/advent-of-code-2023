use std::fs;

fn main() {
    let codes = fs::read_to_string("codes.txt").unwrap();
    let mut calibration_values = Vec::new();

    for code in codes.lines() {
        let first_num_char = code.chars().find(|c| c.is_numeric()).unwrap();
        let last_num_char = code.chars().rev().find(|c| c.is_numeric()).unwrap();
        calibration_values.push(first_num_char.to_digit(10).unwrap() * 10 + last_num_char.to_digit(10).unwrap());
    }

    let sum: u32 = calibration_values.iter().sum();

    println!("Sum is: {}", sum);
}
