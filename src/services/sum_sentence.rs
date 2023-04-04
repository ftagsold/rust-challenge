use std::cmp::Reverse;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn sum_of_digits(s: &str) -> u32 {
    s.chars().filter_map(|c| c.to_digit(10)).sum()
}

pub async fn sum_sentence(file_path: &str) -> String {
    let mut i = 0;

    let mut result = Vec::new();

    let file = File::open(file_path).unwrap();

    let split = ['.', ';', '?', '!'].as_ref();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        for sentence in line.split(split) {
            let sum = sum_of_digits(sentence);

            result.push((sum, i));

            i += 1;
        }
    }

    result.sort_by_key(|(s, _)| Reverse(*s));

    let mut result = result.into_iter().take(10).collect::<Vec<_>>();
    result.sort_by_key(|(_, i)| *i);

    result
        .into_iter()
        .enumerate()
        .map(|(i, (s, _))| (s - i as u32) as u8 as char)
        .collect::<String>()
}
