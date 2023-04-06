use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Result {
    pub secret: String,
    pub sum_digits: u64,
    pub sum_vowels: u64,
}

pub fn parse(file_path: &str) -> Result {
    let mut sum_digits: u64 = 0;
    let mut sum_vowels: u64 = 0;

    let mut top_ten = Vec::new();

    let split = ['.', ';', '?', '!'].as_ref();

    let reader = BufReader::new(File::open(file_path).unwrap());

    reader.lines().for_each(|line| {
        let line = line.unwrap();

        if line.trim().len() > 0 {
            line.split(split).for_each(|sentence| {
                let mut sum_sentence: u64 = 0;

                if sentence.trim().len() > 0 {
                    sentence.chars().for_each(|c| {
                        if let Some(digit) = c.to_digit(10) {
                            sum_sentence += digit as u64;
                            sum_digits += digit as u64;
                        } else {
                            match c {
                                'a' => sum_vowels += 2,
                                'A' => sum_vowels += 2,
                                'e' => sum_vowels += 4,
                                'E' => sum_vowels += 4,
                                'i' => sum_vowels += 8,
                                'I' => sum_vowels += 8,
                                'o' => sum_vowels += 16,
                                'O' => sum_vowels += 16,
                                'u' => sum_vowels += 32,
                                'U' => sum_vowels += 32,
                                _ => (),
                            }
                        }
                    });

                    if top_ten.len() < 10 {
                        top_ten.push(sum_sentence);
                    } else {
                        let min = top_ten.iter().min().unwrap();

                        if sum_sentence > *min {
                            let index = top_ten.iter().position(|x| x == min);

                            top_ten.remove(index.unwrap());
                            top_ten.push(sum_sentence);
                        }
                    }
                }
            });
        }
    });

    Result {
        sum_digits,
        sum_vowels,
        secret: top_ten
            .iter()
            .enumerate()
            .map(|(i, s)| (s - i as u64) as u8 as char)
            .collect::<String>(),
    }
}
