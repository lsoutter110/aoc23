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

struct Hand {
    bid: i64,
    strength: usize,
}

const CARD_ORDER: &'static str = "23456789TJQKA";
const CARD_ORDER2: &'static str = "J23456789TQKA";

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
    let hand_type: usize = match (occur[0], occur[1]) {
        (5, _) => 6,
        (4, _) => 5,
        (3, 2) => 4,
        (3, _) => 3,
        (2, 2) => 2,
        (2, _) => 1,
        _ => 0,
    };

    // Calculate strength
    let mut strength = hand_type;
    for c in cards.chars() {
        strength = strength*CARD_ORDER.len() + (if joker { CARD_ORDER2 } else { CARD_ORDER }).find(c).unwrap();
    }
    return strength;
}

fn part1(filename: &str) -> i64 {
    let mut hands: Vec<Hand> = file::read_to_lines(filename).iter().map(|l| {
        let (cards, bid) = parse!(l, "{} {}");
        Hand { bid, strength: calculate_strength(cards, false) }
    }).collect();
    hands.sort_unstable_by(|a, b| a.strength.cmp(&b.strength));
    return hands.iter().enumerate().map(|(i,h)| (i+1) as i64 * h.bid).sum();
}

fn part2(filename: &str) -> i64 {
    let mut hands: Vec<Hand> = file::read_to_lines(filename).iter().map(|l| {
        let (cards, bid) = parse!(l, "{} {}");
        Hand { bid, strength: calculate_strength(cards, true) }
    }).collect();
    hands.sort_unstable_by(|a, b| a.strength.cmp(&b.strength));
    return hands.iter().enumerate().map(|(i,h)| (i+1) as i64 * h.bid).sum();
}