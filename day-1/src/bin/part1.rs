use std::{fs::read_to_string, path::Path};

fn main() {
    let input = match read_to_string(Path::new("src/input1.txt")) {
        Err(err) => panic!("couldn't open input file: {}", err),
        Ok(value) => value,
    };

    let mut sum = 0;
    for line in input.split('\n') {
        let mut first = 0;
        let mut last = 0;

        for ch in line.chars() {
            if !ch.is_digit(10) {
                continue;
            }

            if first == 0 {
                first = ch.to_digit(10).unwrap();
            }
            last = ch.to_digit(10).unwrap()
        }

        sum += first * 10 + last;
    }

    println!("sum: {}", sum);
}
