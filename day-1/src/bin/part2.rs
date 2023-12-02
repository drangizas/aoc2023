use std::{collections::HashMap, fs::read_to_string, path::Path};

fn main() {
    let input = match read_to_string(Path::new("src/input2.txt")) {
        Err(err) => panic!("couldn't open input file: {}", err),
        Ok(value) => value,
    };

    let allowed_words_map: HashMap<&str, u8> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .into_iter()
    .collect();

    let mut sum = 0;
    for line in input.split('\n') {
        let mut first = 0;
        let mut last = 0;

        for (idx, ch) in line.char_indices() {
            if let Some(digit_value) = ch.to_digit(10) {
                if first == 0 {
                    first = digit_value
                }
                last = digit_value;
            } else {
                for (key, value) in allowed_words_map.iter() {
                    if line[idx..].starts_with(key) {
                        if first == 0 {
                            first = *value as u32;
                        }
                        last = *value as u32;
                    }
                }
            }
        }

        println!("{} {}", first, last);
        sum += first * 10 + last;
    }

    println!("sum: {}", sum);
}
