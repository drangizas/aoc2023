use std::fs;
use std::path::Path;

fn main() {
    let file = fs::read_to_string(Path::new("src/input1.txt")).expect("Could not read the input file");
    let possible_cubes = [
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ];

    let mut sum_of_valid_ids: u32 = 0;
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
        if is_game_valid(line[curr_idx..].to_string(), &possible_cubes.into()) {
            println!("found valid game: {}", game_id);
            sum_of_valid_ids += game_id;
        }
    }

    println!("sum of valid ids: {}", sum_of_valid_ids);
}

fn is_game_valid(game: String, possible_cubes: &Vec<(&str, u32)>) -> bool {
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
            if cube.ends_with(possible_cube.0) && amount > possible_cube.1 {
                return false;
            }
        }
    }

    true
}
