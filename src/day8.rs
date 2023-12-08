use aoc_util::*;
use std::time::Instant;
use prse::{self, Parse, parse, try_parse};
use std::collections::HashMap;
use std::iter::zip;

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

const LETTERS: i64 = 'Z' as i64 + 1;

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

fn end(node: i64) -> bool {
    return node % LETTERS == lindex('Z');
}

// fn node_to

fn part2(filename: &str) -> i64 {
    let mut lines = file::read_to_lines(filename).into_iter();
    let directions = lines.next().unwrap();
    let _ = lines.next();
    let mut tree: HashMap<i64, (i64, i64)> = HashMap::new();

    for l in lines {
        let (node, lhs, rhs) = parse!(l, "{} = ({}, {})");
        tree.insert(node_to_n(node), (node_to_n(lhs), node_to_n(rhs)));
    }

    let mut nodes: Vec<i64> = tree.keys().filter(|k| (**k) % LETTERS == lindex('A')).cloned().collect();
    let mut last_z: Vec<i64> = vec![0; nodes.len()];
    let mut z_cyc: Vec<i64> = vec![0; nodes.len()];
    let mut z_cyc_n = 0;
    let mut count = 0;
    while z_cyc_n != nodes.len() {
        for c in directions.chars() {
            for i in 0..nodes.len() {
                if end(nodes[i]) {
                    if last_z[i] == 0  {
                        last_z[i] = count;
                    } else if z_cyc[i] == 0 {
                        z_cyc[i] = count - last_z[i];
                        z_cyc_n += 1;
                    }
                }
            }
            if z_cyc_n == nodes.len() { break; }
            count += 1;
            for i in 0..nodes.len() {
                match c {
                    'L' => nodes[i] = tree[&nodes[i]].0,
                    'R' => nodes[i] = tree[&nodes[i]].1,
                    _ => panic!("Bad direction"),
                }
            }
        }
    }
    // all cycle to start, find lcm
    let mut factors: HashMap<i64,i64> = HashMap::new();
    for z in z_cyc {
        let pf = prime_factors(z);
        for (k, v) in pf {
            if factors.contains_key(&k) {
                *factors.get_mut(&k).unwrap() = std::cmp::max(factors[&k], v);
            } else {
                factors.insert(k, v);
            }
        }
    }
    return factors.into_iter().fold(1, |a, (k,v)| a * k.pow(v as u32));
}

fn prime_factors(n: i64) -> HashMap<i64,i64> {
    let mut factors: HashMap<i64,i64> = HashMap::new();
    let mut a = n;
    for i in 2..=n {
        if a % i == 0 {
            let mut c = 0;
            while a % i == 0 {
                c += 1;
                a /= i;
            }
            factors.insert( i, c);
            if a == 1 { break; }
        }
    }
    return factors;
}

// 35575074889 low