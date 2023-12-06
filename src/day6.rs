use aoc_util::*;
use std::time::Instant;
use std::iter::zip;
use prse::{self, Parse, parse, try_parse};
use regex::Regex;

pub fn main(filename: &str) {
    let start = Instant::now();
    println!("Part 1:");
    part1(filename);
    println!("({:?})", start.elapsed());

    let start = Instant::now();
    println!("Part 2:");
    part2(filename);
    println!("({:?})", start.elapsed());
}

fn part1(filename: &str) {
    let [times, dists, ..] = &file::read_to_lines(filename)[..] else { panic!("Bad input") };
    let times: Vec<i32> = parse!(Regex::new(r" +").unwrap().replace_all(times, " "), "Time: {: :}");
    let dists: Vec<i32> = parse!(Regex::new(r" +").unwrap().replace_all(dists, " "), "Distance: {: :}");
    let prod: i32 = zip(times, dists).map(|(l,r)| n(l as f64, r as f64) as i32).product();
    println!("Product is {prod}");
}

fn n(l: f64, r: f64) -> f64 {
    return f64::ceil((l+f64::sqrt(l.powi(2)-4.0*r))/2.0 - 1.0)
        - f64::floor((l-f64::sqrt(l.powi(2)-4.0*r))/2.0 + 1.0)
        + 1.0;
}

fn part2(filename: &str) {
    let [times, dists, ..] = &file::read_to_lines(filename)[..] else { panic!("Bad input") };
    let times: Vec<i64> = parse!(Regex::new(r" +").unwrap().replace_all(times, ""), "Time:{: :}");
    let dists: Vec<i64> = parse!(Regex::new(r" +").unwrap().replace_all(dists, ""), "Distance:{: :}");
    let prod: i64 = zip(times, dists).map(|(l,r)| n(l as f64, r as f64) as i64).product();
    println!("Product is {prod}");
}
