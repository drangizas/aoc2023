use std::{fs, path::Path};

fn main() {
    let file =
        fs::read_to_string(Path::new("src/input1.txt")).expect("Could not read the input file");

    let lines = file
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|l| l.split_ascii_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut number_of_ways: Vec<usize> = vec![];
    for idx in 1..lines[0].len() {
        let goal = lines[1][idx]
            .parse::<usize>()
            .expect("Could not parse goal amount");
        let time = lines[0][idx]
            .parse::<usize>()
            .expect("Could not parse time amount");

        let mut possible_ways = 0_usize;
        for time_idx in 1..time {
            let distance = (time - time_idx) * time_idx;

            if distance > goal {
                possible_ways += 1;
            }
        }

        if possible_ways > 0 {
            number_of_ways.push(possible_ways);
        }
    }

    println!(
        "Possible ways: {:?}, result: {:?}",
        number_of_ways,
        number_of_ways.iter().fold(1, |acc, n| acc * n)
    )
}
