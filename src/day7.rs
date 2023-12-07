use aoc_util::*;
use std::time::Instant;
use prse::{self, Parse, parse, try_parse};

pub fn main(filename: &str) {
    println!("Part 1:");
    let start = Instant::now();
    let result = part1(filename);
    let elapsed = start.elapsed();
    println!("{result} ({:?})", elapsed);

    println!("Part 2:");
    let start = Instant::now();
    let result = part2(filename);
    let elapsed = start.elapsed();
    println!("{result} ({:?})", elapsed);
}

enum HandType {
    FiveKind = 6,
    FourKind = 5,
    FullHouse = 4,
    ThreeKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

// #[derive(Parse)]
// enum Card {
//     #[prse="A"] A = 12,
//     #[prse="K"] K = 11,
//     #[prse="Q"] Q = 10,
//     #[prse="J"] J = 9,
//     #[prse="T"] T = 8,
//     #[prse="9"] N9 = 7,
//     #[prse="8"] N8 = 6,
//     #[prse="7"] N7 = 5,
//     #[prse="6"] N6 = 4,
//     #[prse="5"] N5 = 3,
//     #[prse="4"] N4 = 2,
//     #[prse="3"] N3 = 1,
//     #[prse="2"] N2 = 0,
// }

struct Hand {
    cards: String,
    bid: i64,
    strength: usize,
}

const CARD_ORDER: &'static str = "23456789TJQKA";
const CARD_ORDER2: &'static str = "J23456789TQKA";

impl Hand {
    fn new(cards: String, bid: i64, joker: bool) -> Hand {
        let strength = Hand::calculate_strength(&cards[..], joker);
        return Hand { cards, bid, strength };
    }

    fn calculate_strength(cards: &str, joker: bool) -> usize {
        // Find type
        let mut occur = [0; CARD_ORDER.len()];
        let mut jokers = 0;
        for c in cards.chars() {
            match (joker, c) {
                (true, 'J') => jokers += 1,
                (true, _) => occur[CARD_ORDER2.find(c).unwrap()] += 1,
                (false, _) => occur[CARD_ORDER.find(c).unwrap()] += 1,
            }
        }
        occur.sort_by(|a, b| b.cmp(a));
        occur[0] += jokers;
        let hand_type = match (occur[0], occur[1]) {
            (5, _) => HandType::FiveKind,
            (4, _) => HandType::FourKind,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeKind,
            (2, 2) => HandType::TwoPair,
            (2, _) => HandType::OnePair,
            _ => HandType::HighCard,
        };

        // Calculate strength
        let mut strength = hand_type as usize;
        for c in cards.chars() {
            strength = strength*CARD_ORDER.len() + (if joker { CARD_ORDER2 } else { CARD_ORDER }).find(c).unwrap();
        }
        return strength;
    }
}

impl std::cmp::PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.strength == other.strength
    }

    fn ne(&self, other: &Self) -> bool {
        self.strength != other.strength
    }
}

impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, b: &Self) -> Option<std::cmp::Ordering> {
        self.strength.partial_cmp(&b.strength)
    }
}

impl std::cmp::Eq for Hand {
    fn assert_receiver_is_total_eq(&self) {
        
    }
}

impl std::cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.strength.cmp(&other.strength)
    }

    fn clamp(self, min: Self, max: Self) -> Self
        where
            Self: Sized,
            Self: PartialOrd, {
        Hand {
            cards: self.cards,
            bid: self.bid,
            strength: self.strength.clamp(min.strength, max.strength),
        }
    }
}

fn part1(filename: &str) -> i64 {
    let mut hands: Vec<Hand> = file::read_to_lines(filename).iter().map(|l| {
        let (cards, bid) = parse!(l, "{} {}");
        Hand::new(cards, bid, false)
    }).collect();
    hands.sort_unstable();
    return hands.iter().enumerate().map(|(i,h)| (i+1) as i64 * h.bid).sum();
}

fn part2(filename: &str) -> i64 {
    let mut hands: Vec<Hand> = file::read_to_lines(filename).iter().map(|l| {
        let (cards, bid) = parse!(l, "{} {}");
        Hand::new(cards, bid, true)
    }).collect();
    hands.sort_unstable();
    return hands.iter().enumerate().map(|(i,h)| (i+1) as i64 * h.bid).sum();
}

// too low 248640391