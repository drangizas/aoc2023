use std::{fs, path::Path};
use std::{str, vec};
use std::collections::HashMap;

fn main() {
    let file =
        fs::read_to_string(Path::new("src/input2.txt")).expect("Could not read the input file");

    let mut points_sum = 0_u32;
    let mut scratchcards_won_map: HashMap<u32, u32> = HashMap::new();

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

        if let Some(games_won) = scratchcards_won_map.get_mut(&game_id) {
            *games_won = games_won.clone() + 1_u32;
        } else {
            scratchcards_won_map.insert(game_id, 1_u32);
        }

        for nr_won in 1..=matching_numbers.len() {
            let curr_key = game_id + nr_won as u32;

            *scratchcards_won_map.entry(curr_key).or_insert(0) += *scratchcards_won_map.get_mut(&game_id).unwrap();
        }

        println!("curr game: {}, matching numbers: {}, games won: {:?}", game_id, matching_numbers.len(), scratchcards_won_map.get(&game_id));
        println!("----------------------------------------------------");
    }

    println!("Calculated total sum: {}", points_sum);
    println!("Scratchards in total: {}", scratchcards_won_map.values().sum::<u32>());
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
