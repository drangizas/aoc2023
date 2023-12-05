use std::{fs, path::Path};

const SYMBOL_TO_SKIP: char = '.';

fn main() {
    let file =
        fs::read_to_string(Path::new("src/input1.txt")).expect("Could not read the input file");

    let mut chars_matrix: Vec<Vec<char>> = create_2d_matrix(file);
    let mut sum_of_numbers = 0;

    for row_idx in 0..chars_matrix.len() {
        let row = chars_matrix[row_idx].clone();

        for (col_idx, col) in row.iter().enumerate() {
            if col != &SYMBOL_TO_SKIP && !col.is_alphanumeric() {
                let numbers_around = find_numbers_around(row_idx, col_idx, &mut chars_matrix);
                sum_of_numbers += numbers_around.iter().sum::<i32>();

                println!(
                    "[{row_idx}][{col_idx}] {col}: numbers_around: {:?}",
                    numbers_around
                );
            }
        }
    }

    println!("sum of valid numbers: {}", sum_of_numbers);
}

fn create_2d_matrix(file: String) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = vec![];

    for line in file.split('\n') {
        matrix.push(Vec::from_iter(line.chars()));
    }

    matrix
}

fn find_numbers_around(
    row_idx: usize,
    col_idx: usize,
    chars_matrix: &mut Vec<Vec<char>>,
) -> Vec<i32> {
    let possible_number_pos: [(i8, i8); 8] = [
        (-1, 1),
        (0, 1),
        (1, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    let mut numbers_found: Vec<i32> = vec![];
    for coordinate in possible_number_pos {
        let curr_row_idx: i32 = (row_idx as i32) + (coordinate.1 as i32);
        if curr_row_idx < 0 {
            continue;
        }

        if let Some(row) = chars_matrix.get_mut(curr_row_idx as usize) {
            let curr_col_idx: i32 = (col_idx as i32) + (coordinate.0 as i32);
            if curr_col_idx < 0 {
                continue;
            }

            if let Some(col) = row.get(curr_col_idx as usize) {
                if col.is_digit(10) {
                    numbers_found.push(parse_full_number(row, curr_col_idx as usize));
                }
            }
        }
    }

    numbers_found
}

fn parse_full_number(row: &mut Vec<char>, curr_col_idx: usize) -> i32 {
    let mut start_idx: usize = curr_col_idx;
    while start_idx > 0 && row.get(start_idx - 1).map_or(false, |ch| ch.is_digit(10)) {
        start_idx -= 1;
    }

    let mut full_number: i32 = 0;
    for idx in start_idx..row.len() {
        let ch = row[idx];

        if ch.is_digit(10) {
            full_number = full_number * 10 + ch.to_digit(10).map_or(0, |d| d as i32);

            row[idx] = SYMBOL_TO_SKIP;
        } else {
            break;
        }
    }

    full_number
}
