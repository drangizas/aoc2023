use std::{
    collections::HashMap,
    fs,
    path::Path,
};
use std::cmp::Ordering;

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
        'J' => 1_u8,
        '2' => 2_u8,
        '3' => 3_u8,
        '4' => 4_u8,
        '5' => 5_u8,
        '6' => 6_u8,
        '7' => 7_u8,
        '8' => 8_u8,
        '9' => 9_u8,
        'T' => 10_u8,
        'Q' => 11_u8,
        'K' => 12_u8,
        'A' => 13_u8,
        _ => 0_u8,
    }
}

fn get_hand_type_weight(hand: &HandType) -> u8 {
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

fn compare_hand_chars_weight(hand1: &str, hand2: &str, curr_idx: usize) -> Ordering {
    let chars1 = hand1.chars().collect::<Vec<char>>();
    let chars2 = hand2.chars().collect::<Vec<char>>();

    let diff: isize = get_card_weight(chars1[curr_idx]) as isize - get_card_weight(chars2[curr_idx]) as isize;
    if diff >= 1 {
        Ordering::Less
    } else if diff <= -1 {
        Ordering::Greater
    } else {
        compare_hand_chars_weight(hand1, hand2, curr_idx + 1)
    }
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

    hands.sort_by(|_self, _other| {
        let self_rank = get_hand_type_weight(&_self.hand_type);
        let other_rank = get_hand_type_weight(&_other.hand_type);

        if self_rank > other_rank {
            Ordering::Less
        } else if self_rank < other_rank {
            Ordering::Greater
        } else {
            compare_hand_chars_weight(&_self.cards, &_other.cards, 0)
        }
    });

    let total_size = hands.len();
    for (idx, hand) in hands.iter_mut().enumerate() {
        hand.rank = total_size - idx;
    }

    println!("{:?}", hands);
    println!("Total winnings: {:?}", hands.iter().fold(0, |acc, hand| acc + (hand.bid * hand.rank)));
}
