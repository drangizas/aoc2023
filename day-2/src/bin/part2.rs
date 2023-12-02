use std::fs;
use std::path::Path;

#[derive(Debug)]
struct GameMaxes {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let file = fs::read_to_string(Path::new("src/input2.txt")).expect("Could not read the input file");
    let possible_cubes = [
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ];

    let mut sum_of_valid_ids: u32 = 0;
    let mut sum_of_game_maxes = 0;
    for line in file.split('\n') {
        // Starts at 4 to skip "Game"
        let mut curr_idx = 5;

        let mut game_id: u32 = 0;
        for char in line[curr_idx..].chars() {
            curr_idx += 1;

            if char == ':' { break; }

            if let Some(digit) = char.to_digit(10) {
                game_id = game_id * 10 + digit;
            }
        }

        // Parse cubes
        let (is_valid, game_maxes) = is_game_valid(line[curr_idx..].to_string(), &possible_cubes.into());
        if is_valid {
            sum_of_valid_ids += game_id;
        }
        sum_of_game_maxes += (game_maxes.red * game_maxes.green * game_maxes.blue);
    }

    println!("sum of valid ids: {}", sum_of_valid_ids);
    println!("sum of game maxes: {}", sum_of_game_maxes);
}

fn is_game_valid(game: String, possible_cubes: &Vec<(&str, u32)>) -> (bool, GameMaxes) {
    let mut is_valid = true;
    let mut games_maxes = GameMaxes { red: 0, green: 0, blue: 0 };

    for cubes in game.split([',', ';']) {
        let cube = cubes.trim();

        let mut amount: u32 = 0;
        for char in cube.chars() {
            if let Some(digit) = char.to_digit(10) {
                amount = amount * 10 + digit;
            } else {
                break;
            }
        }

        for possible_cube in possible_cubes {
            if cube.ends_with(possible_cube.0) {
                if amount > possible_cube.1 {
                    is_valid = false;
                }

                if possible_cube.0 == "red" && amount > games_maxes.red {
                    games_maxes.red = amount;
                } else if possible_cube.0 == "green" && amount > games_maxes.green {
                    games_maxes.green = amount;
                } else if possible_cube.0 == "blue" && amount > games_maxes.blue {
                    games_maxes.blue = amount;
                }
            }
        }
    }

    (is_valid, games_maxes)
}
