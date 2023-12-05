use aoc_util::*;
use prse::{self, Parse, parse, try_parse};
use std::time::Instant;

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

struct IdRange {
    s: i64,
    e: i64,
}

fn part2(filename: &str) {
    let lines = file::read_to_lines(filename);
    let seeds: Vec<i64> = parse!(lines[0], "seeds: {: :}");
    let mut ids: Vec<IdRange> = seeds.chunks(2).map(|c| IdRange { s: c[0], e: c[0]+c[1]}).collect();
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
    
    for m in maps {
        ids = fold_ids(ids, m);
    }
    let min_loc = ids.iter().map(|id| id.s).min().unwrap();
    println!("Minimum location = {min_loc}");
}

fn fold_ids(mut ids: Vec<IdRange>, mut maps: Vec<MapRange>) -> Vec<IdRange> {
    use std::cmp::min;
    let mut new_ids = Vec::new();

    ids.sort_by(|a, b| a.s.cmp(&b.s));
    maps.sort_by(|a, b| a.source.cmp(&b.source));

    let mut ids = ids.into_iter();
    let mut maps = maps.into_iter();
    let mut id = ids.next().unwrap();
    let mut map = maps.next().unwrap();
    let mut ptr = id.s;

    loop {
        while ptr >= map.source + map.len {
            map = match maps.next() {
                Some(m) => m,
                None => {
                    //push leftover IDs and quit
                    new_ids.push(IdRange { s: ptr, e: id.e });
                    new_ids.extend(ids);
                    return new_ids;
                }
            }
        }
        if ptr < map.source {
            // ptr   v---->v
            // ids [-~~~~~~---]
            // map         [----]
            new_ids.push(IdRange { s: ptr, e: min(map.source, id.e) });
            ptr = min(map.source, id.e);
        } else {
            // ptr     v------>v
            // ids     [~~~~~~~---]
            // map   [---------]
            let shift = map.dest - map.source;
            new_ids.push(IdRange { s: ptr + shift, e: min(map.source + map.len, id.e) + shift });
            ptr = min(map.source + map.len, id.e);
        }
        //run out of IDs
        if ptr == id.e {
            id = match ids.next() {
                Some(i) => i,
                None => return new_ids,
            };
            ptr = id.s
        }
    }
}