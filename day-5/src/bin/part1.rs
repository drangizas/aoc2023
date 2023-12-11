use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
struct FromToRange(usize, usize, usize);

#[derive(Debug)]
struct CategoryInfo {
    maps_from: Category,
    maps_to: Category,
    map: Vec<FromToRange>,
}

impl CategoryInfo {
    fn new(maps_from: Category, maps_to: Category, map: Vec<FromToRange>) -> Self {
        Self {
            maps_from,
            maps_to,
            map,
        }
    }

    fn get_mapping_of(&self, idx: &usize) -> usize {
        if let Some(mapping) = self.map.iter().find(|&m| m.0 <= *idx && m.0 + m.2 > *idx) {
            let diff = *idx - mapping.0;

            mapping.1 + diff
        } else {
            *idx
        }
    }
}

const MAPPING_CHAIN: [(Category, Category); 7] = [
    (Category::Seed, Category::Soil),
    (Category::Soil, Category::Fertilizer),
    (Category::Fertilizer, Category::Water),
    (Category::Water, Category::Light),
    (Category::Light, Category::Temperature),
    (Category::Temperature, Category::Humidity),
    (Category::Humidity, Category::Location),
];

fn main() {
    let file =
        fs::read_to_string(Path::new("src/input1.txt")).expect("Could not read the input file");

    let lines = file.split('\n').collect::<Vec<&str>>();
    let seeds: Vec<usize> = lines.get(0).expect("File is malformed (no first line)")[7..]
        .split_ascii_whitespace()
        .filter_map(|digit| digit.parse::<usize>().ok())
        .collect();
    let mut mappings: Vec<CategoryInfo> = vec![];

    let mut idx = 2_usize;
    while idx < lines.len() - 1 {
        if lines[idx].is_empty() {
            idx += 1;
            continue;
        }

        if lines[idx].ends_with("map:") {
            let (from, to) = parse_map_from_to(&lines[idx][0..lines[idx].find(' ').unwrap()]);
            idx += 1;

            let mut map: Vec<FromToRange> = vec![];

            while idx < lines.len() {
                if lines[idx].is_empty() {
                    idx += 1;
                    break;
                }

                let numbers = lines[idx].split_ascii_whitespace().collect::<Vec<&str>>();
                map.push(FromToRange(
                    numbers[1].parse::<usize>().unwrap(),
                    numbers[0].parse::<usize>().unwrap(),
                    numbers[2].parse::<usize>().unwrap(),
                ));

                idx += 1;
            }

            mappings.push(CategoryInfo::new(from, to, map));
        }
    }

    let mut lowest_loc = usize::MAX;
    for seed in seeds {
        let location = find_seed_location(&seed, &mappings);
        if location < lowest_loc {
            lowest_loc = location
        }
    }

    println!("lowest location id: {}", lowest_loc);
}

fn find_seed_location(seed: &usize, mappings: &Vec<CategoryInfo>) -> usize {
    let mut path: Vec<usize> = vec![*seed];

    for mapping in MAPPING_CHAIN {
        let matching_mapping = mappings
            .iter()
            .find(|&m| m.maps_from == mapping.0 && m.maps_to == mapping.1)
            .unwrap();

        path.push(matching_mapping.get_mapping_of(path.last().unwrap()));
    }

    println!("seed: {}, path: {:?}", seed, path);

    *path.last().unwrap()
}

fn parse_map_from_to(map_str: &str) -> (Category, Category) {
    let words = map_str.split('-').collect::<Vec<&str>>();

    return (
        switch_string_to_category(words[0]),
        switch_string_to_category(words[2]),
    );
}

fn switch_string_to_category(cat: &str) -> Category {
    match cat {
        "seed" => Category::Seed,
        "soil" => Category::Soil,
        "fertilizer" => Category::Fertilizer,
        "water" => Category::Water,
        "light" => Category::Light,
        "temperature" => Category::Temperature,
        "humidity" => Category::Humidity,
        "location" => Category::Location,
        _ => panic!("unknown category string provided"),
    }
}
