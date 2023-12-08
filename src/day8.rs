use aoc_util::*;
use std::time::Instant;
use prse::{self, Parse, parse, try_parse};
use std::collections::HashMap;

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

const LETTERS: i64 = ('Z' as i64 + 1);

fn node_to_n(code: &str) -> i64 {
    code.chars().fold(0, |a,c| a*LETTERS + lindex(c))
}

fn lindex(c: char) -> i64 {
    return c as i64 - 'A' as i64;
}

fn part1(filename: &str) -> i64 {
    let mut lines = file::read_to_lines(filename).into_iter();
    let directions = lines.next().unwrap();
    let _ = lines.next();
    let mut tree: HashMap<i64, (i64, i64)> = HashMap::new();

    for l in lines {
        let (node, lhs, rhs) = parse!(l, "{} = ({}, {})");
        tree.insert(node_to_n(node), (node_to_n(lhs), node_to_n(rhs)));
    }

    let mut node = node_to_n("AAA");
    let mut count = 0;
    let end: i64 = node_to_n("ZZZ");
    while node != end {
        for c in directions.chars() {
            count += 1;
            match c {
                'L' => node = tree[&node].0,
                'R' => node = tree[&node].1,
                _ => panic!("Bad direction"),
            }
            if node == end { break; }
        }
    }
    return count;
}

fn end(nodes: &Vec<i64>) {
    return nodes.iter().
}

fn part2(filename: &str) -> i64 {
    let mut lines = file::read_to_lines(filename).into_iter();
    let directions = lines.next().unwrap();
    let _ = lines.next();
    let mut tree: HashMap<i64, (i64, i64)> = HashMap::new();

    for l in lines {
        let (node, lhs, rhs) = parse!(l, "{} = ({}, {})");
        tree.insert(node_to_n(node), (node_to_n(lhs), node_to_n(rhs)));
    }

    let mut nodes: Vec<i64> = tree.keys().filter(|k| *k % LETTERS == lindex('A')).cloned().collect();
    let mut count = 0;
    let end: i64 = node_to_n("ZZZ");
    while node != end {
        for c in directions.chars() {
            count += 1;
            match c {
                'L' => node = tree[&node].0,
                'R' => node = tree[&node].1,
                _ => panic!("Bad direction"),
            }
            if node == end { break; }
        }
    }
    return count;
}
