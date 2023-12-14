use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

#[derive(Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    cards: String,
    hand_type: HandType,
    rank: usize,
    bid: usize,
}

fn get_card_weight(card: char) -> u8 {
    match card {
        '2' => 1_u8,
        '3' => 2_u8,
        '4' => 3_u8,
        '5' => 4_u8,
        '6' => 5_u8,
        '7' => 6_u8,
        '8' => 7_u8,
        '9' => 8_u8,
        'T' => 9_u8,
        'J' => 10_u8,
        'Q' => 11_u8,
        'K' => 12_u8,
        'A' => 13_u8,
        _ => 0_u8,
    }
}

fn get_hand_type_weight(hand: HandType) -> u8 {
    match hand {
        HandType::HighCard => 1_u8,
        HandType::OnePair => 2_u8,
        HandType::TwoPair => 3_u8,
        HandType::ThreeOfAKind => 4_u8,
        HandType::FullHouse => 5_u8,
        HandType::FourOfAKind => 6_u8,
        HandType::FiveOfAKind => 7_u8,
    }
}

fn find_hand_type(hand: &str) -> HandType {
    let char_set: HashMap<char, usize> = hand.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    println!("{} -> {:?}", hand, char_set);
    if char_set.keys().len() == 1 {
        return HandType::FiveOfAKind;
    }

    if char_set.values().find(|&v| *v == 4_usize).is_some() {
        return HandType::FourOfAKind;
    }

    if char_set.values().find(|&v| *v == 3_usize).is_some()
        && char_set.values().find(|&v| *v == 2_usize).is_some()
    {
        return HandType::FullHouse;
    }

    if char_set.values().find(|&v| *v == 3_usize).is_some() {
        return HandType::ThreeOfAKind;
    }

    if char_set.keys().filter(|k| char_set[k] == 2_usize).count() == 2 {
        return HandType::TwoPair;
    }

    if char_set.values().find(|&v| *v == 2_usize).is_some() {
        return HandType::OnePair;
    }

    HandType::HighCard
}

fn main() {
    let file =
        fs::read_to_string(Path::new("src/input1.txt")).expect("Could not read the input file");

    let cards_and_bid = file
        .split('\n')
        .map(|l| l.split_ascii_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut hands: Vec<Hand> = vec![];
    for card_and_bid in cards_and_bid.iter() {
        hands.push(Hand {
            cards: card_and_bid[0].into(),
            hand_type: find_hand_type(&card_and_bid[0]),
            bid: card_and_bid[1].parse().expect("Could not parse bid"),
            rank: 0,
        })
    }

    println!("{:?}", hands);
}
