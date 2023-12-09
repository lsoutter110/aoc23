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
    let seqs: Vec<Vec<i64>> = file::read_to_lines(filename).iter().map(|l| parse!(l, "{: :}")).collect();
    return seqs.iter().map(get_next).sum();
}

fn part2(filename: &str) -> i64 {
    let seqs: Vec<Vec<i64>> = file::read_to_lines(filename).iter().map(|l| parse!(l, "{: :}")).collect();
    return seqs.iter().map(get_prev).sum();
}

fn get_next(seq: &Vec<i64>) -> i64 {
    let mut diff = Vec::new();
    for i in 0..seq.len()-1 {
        diff.push(seq[i+1]-seq[i]);
    }
    if diff.iter().all(|i| *i == 0) {
        return seq[0]; // all same val
    }
    return seq[seq.len()-1] + get_next(&diff);
}

fn get_prev(seq: &Vec<i64>) -> i64 {
    let mut diff = Vec::new();
    for i in 0..seq.len()-1 {
        diff.push(seq[i+1]-seq[i]);
    }
    if diff.iter().all(|i| *i == 0) {
        return seq[0]; // all same val
    }
    return seq[0] - get_prev(&diff);
}