use std::fs::File;
use std::io::{BufRead, BufReader};

pub async fn sum_file(file_path: &str) -> u32 {
    let mut result = 0;

    let file = File::open(file_path).unwrap();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        for c in line.chars() {
            if let Some(digit) = c.to_digit(10) {
                result += digit;
            }
        }
    }

    result
}
