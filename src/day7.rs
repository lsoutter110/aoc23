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

fn part1(filename: &str) -> i64 {
    println!("day 7");
    0
}

fn part2(filename: &str) -> i64 {
    0
}
