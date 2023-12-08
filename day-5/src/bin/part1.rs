use std::fs;
use std::path::Path;

enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

struct CategoryInfo {
    c_type: Category,
    maps_to: Category,
    map: Vec<(usize, usize)>,
}

fn main() {
    let file =
        fs::read_to_string(Path::new("src/input1.txt")).expect("Could not read the input file");

    let mut seeds: Vec<usize> = vec![];

    for line in file.split('\n') {
        if line.is_empty() { continue; }

        if line.starts_with("seeds:") {
            seeds = line[7..].split_ascii_whitespace().filter_map(|digit| digit.parse::<usize>().ok()).collect()
        }

        if line.ends_with("map:") {
            println!("{}", line);
        }
    }

    println!("seeds: {:?}", seeds)
}
