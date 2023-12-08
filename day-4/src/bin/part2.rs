use std::collections::HashMap;
use std::{fs, path::Path};
use std::{str, vec};

fn main() {
    let file =
        fs::read_to_string(Path::new("src/input2.txt")).expect("Could not read the input file");

    let mut points_sum = 0_u32;
    let mut scratchcards_won_map: HashMap<u32, u32> = HashMap::new();
    let mut scratchcards_won_count = 0_u32;

    for line in file.split('\n') {
        let parts = line.split([':', '|']).collect::<Vec<&str>>();

        let game_id = parse_game_id(parts[0]);
        let winning_numbers = parse_game_numbers(parts[1]);
        let my_numbers = parse_game_numbers(parts[2]);

        let mut matching_numbers: Vec<u32> = vec![];
        for my_numb in my_numbers {
            if winning_numbers.contains(&my_numb) {
                matching_numbers.push(my_numb);
            }
        }

        let points_for_game = match matching_numbers.len() {
            0 => 0,
            1 => 1,
            _ => 2_u32.pow((matching_numbers.len() - 1) as u32),
        };

        points_sum += points_for_game;

        for nr_won in 1..=matching_numbers.len() {
            let curr_key = game_id + nr_won as u32;

            let iter_game_value = scratchcards_won_map.get(&curr_key).unwrap_or(&0_u32);
            let curr_game_value = scratchcards_won_map.get(&game_id).unwrap_or(&0_u32);
            println!(
                "{}:{}, {}:{}",
                curr_key, iter_game_value, game_id, curr_game_value
            );

            scratchcards_won_map.insert(curr_key, 1 + (1 * iter_game_value * curr_game_value));
        }
        println!("Current scratchards map: {:?}", scratchcards_won_map);
    }

    scratchcards_won_count = scratchcards_won_map.values().sum();

    println!("Calculated total sum: {}", points_sum);
    println!("Scratchards in total: {}", scratchcards_won_count);
}

fn parse_game_id(data: &str) -> u32 {
    let first_digit_idx = data.rfind(' ').unwrap() + 1;
    let game_id_raw: &str =
        str::from_utf8(&data.as_bytes()[first_digit_idx..]).expect("Could not parse game id bytes");

    let game_id = game_id_raw
        .parse::<u32>()
        .expect("Could not parse game id string to u32");

    game_id
}

fn parse_game_numbers(data: &str) -> Vec<u32> {
    data.split_ascii_whitespace()
        .filter_map(|digits| digits.parse::<u32>().ok())
        .collect()
}
