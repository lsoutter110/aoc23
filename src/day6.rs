use aoc_util::*;
use std::time::Instant;
use std::iter::zip;
use prse::{self, Parse, parse, try_parse};
use regex::Regex;

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

    println!("Part 2 fast:");
    let start = Instant::now();
    let prod = n(44899691.0, 277113618901768.0);
    let duration = start.elapsed();
    println!("{prod} : ({:?})", duration);
}

fn part1(filename: &str) -> i64 {
    let [times, dists, ..] = &file::read_to_lines(filename)[..] else { panic!("Bad input") };
    let times: Vec<i32> = parse!(Regex::new(r" +").unwrap().replace_all(times, " "), "Time: {: :}");
    let dists: Vec<i32> = parse!(Regex::new(r" +").unwrap().replace_all(dists, " "), "Distance: {: :}");
    return zip(times, dists).map(|(l,r)| n(l as f64, r as f64) as i64).product();
}

fn n(l: f64, r: f64) -> f64 {
    return f64::ceil((l+f64::sqrt(l.powi(2)-4.0*r))/2.0 - 1.0)
        - f64::floor((l-f64::sqrt(l.powi(2)-4.0*r))/2.0 + 1.0)
        + 1.0;
}

fn part2(filename: &str) -> i64 {
    let [times, dists, ..] = &file::read_to_lines(filename)[..] else { panic!("Bad input") };
    let times: Vec<i64> = parse!(Regex::new(r" +").unwrap().replace_all(times, ""), "Time:{: :}");
    let dists: Vec<i64> = parse!(Regex::new(r" +").unwrap().replace_all(dists, ""), "Distance:{: :}");
    return zip(times, dists).map(|(l,r)| n(l as f64, r as f64) as i64).product();
}
