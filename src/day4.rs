use aoc_util::*;
use prse::{self, Parse, parse};
use regex::Regex;

pub fn main(filename: &str) {
    println!("Part 1:");
    part1(filename);
    println!("Part 2:");
    part2(filename);
}

#[derive(Parse,Debug)]
#[prse = "Card {num}: {winning: :} | {numbers: :}"]
struct Card {
    num: i32,
    winning: Vec<i32>,
    numbers: Vec<i32>,
}

fn part1(filename: &str) {
    let sum: usize = file::read_to_lines(filename).iter()
        .map(|l| -> Card { parse!(Regex::new(r" +").unwrap().replace_all(l, " "), "{}") })
        .map(|c| (1<<c.numbers.iter().filter(|n| c.winning.contains(n)).count())>>1)
        .sum();
    println!("Sum of points is {}", sum);
}

fn part2(filename: &str) {
    let cards: Vec<Card> = file::read_to_lines(filename).iter()
        .map(|l| -> Card { parse!(Regex::new(r" +").unwrap().replace_all(l, " "), "{}") })
        .collect();
    let sum: usize = (0..cards.len()).map(|i| open_cards(&cards, i)).sum();
    println!("Sum of points is {}", sum);
}

fn open_cards(cards: &[Card], index: usize) -> usize {
    let count = cards[index].numbers.iter().filter(|n| cards[index].winning.contains(n)).count();
    let count2: usize = (index+1..=index+count).map(|i| open_cards(&cards, i)).sum();
    return count + count2;
}