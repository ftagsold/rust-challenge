use std::fs::File;
use std::io::{BufRead, BufReader};

pub async fn sum_vocals(file_path: &str) -> u32 {
    let mut result = 0;

    let file = File::open(file_path).unwrap();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        for c in line.chars() {
            match c {
                'a' => result += 2,
                'A' => result += 2,
                'e' => result += 4,
                'E' => result += 4,
                'i' => result += 8,
                'I' => result += 8,
                'o' => result += 16,
                'O' => result += 16,
                'u' => result += 32,
                'U' => result += 32,
                _ => result += 0,
            }
        }
    }

    result
}
