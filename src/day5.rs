use aoc_util::*;
use prse::{self, Parse, parse, try_parse};

pub fn main(filename: &str) {
    println!("Part 1:");
    part1(filename);
    println!("Part 2:");
    part2(filename);
}

#[derive(Parse,Debug)]
#[prse = "{dest} {source} {len}"]
struct MapRange {
    source: i64,
    dest: i64,
    len: i64,
}

fn part1(filename: &str) {
    let lines = file::read_to_lines(filename);
    let seeds: Vec<i64> = parse!(lines[0], "seeds: {: :}");
    let mut maps: Vec<Vec<MapRange>> = Vec::new();
    let mut cur_range: Vec<MapRange> = Vec::new();
    
    for l in &lines {
        if l.is_empty() {
            if !cur_range.is_empty() {
                maps.push(cur_range);
                cur_range = Vec::new();
            }
        }
        if let Ok(range) = try_parse!(l, "{}") {
            cur_range.push(range);
        }
    }
    let min_loc = seeds.iter().map(|s| map_to_loc(&maps, *s)).min().unwrap();
    println!("Minimum location = {min_loc}");
}

fn part2(filename: &str) {
    let lines = file::read_to_lines(filename);
    let seeds: Vec<i64> = parse!(lines[0], "seeds: {: :}");
    let seeds: Vec<&[i64]> = seeds.chunks(2).collect();
    let mut maps: Vec<Vec<MapRange>> = Vec::new();
    let mut cur_range: Vec<MapRange> = Vec::new();
    
    for l in &lines {
        if l.is_empty() {
            if !cur_range.is_empty() {
                maps.push(cur_range);
                cur_range = Vec::new();
            }
        }
        if let Ok(range) = try_parse!(l, "{}") {
            cur_range.push(range);
        }
    }
    let min_loc = seeds.iter().map(|s| (s[0]..s[0]+s[1]).map(|i| map_to_loc(&maps, i)).min().unwrap()).min().unwrap();
    println!("Minimum location = {min_loc}");
    // for s in seeds {
    //     println!("{s} -> {}", map_to_loc(&maps, s));
    // }
}

fn map_to_loc(maps: &Vec<Vec<MapRange>>, mut i: i64) -> i64 {
    for m in maps {
        let mut i_n = i;
        for r in m {
            if i >= r.source && i < r.source+r.len {
                i_n = i - r.source + r.dest;
            }
        }
        i = i_n;
    }
    return i;
}