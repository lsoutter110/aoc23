use aoc_util::*;
use std::{time::Instant, str::Chars};
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
    let lines = file::read_to_lines(filename);
    let rows: Vec<(&str, Vec<i64>)> = lines.iter().map(|l| parse!(l, "{} {:,:}")).collect();
    
    let mut sum = 0;
    for (springs, groups) in rows {
        let spring_bytes: Vec<u8> = springs.bytes().collect();
        let i = rec_fill(&spring_bytes, &groups[..]);
        // println!("{i}");
        sum += i;
    }
    return sum;
}

fn rec_fill(springs: &[u8], groups: &[i64]) -> i64 {
    // check until 0
    let mut sum = 0;
    let last_group = groups.len() == 1;
    let group = groups[0] as usize;
    // println!("{:?} (group {group}):", springs.iter().map(|c| *c as char).collect::<String>());

    if group > springs.len() || groups.iter().map(|i| *i).reduce(|a,i| i + 1 + a).unwrap() > springs.len() as i64 {
        return 0;
    }

    for i in 0..=(springs.len() - group) {
        // println!("index {i}");
        // unnaccounted for #s
        if springs[0..i].iter().any(|s| *s == '#' as u8) { break; }
        if last_group && springs[i+group..].iter().any(|s| *s == '#' as u8) { continue; }
        // broken springs in group
        if springs[i..i+group].iter().any(|s| *s == '.' as u8) { continue; }
        // no more space (group + spacer overruns the spring list)
        if i+group+1 >= springs.len() && !last_group { break; }
        // missing spacer
        if !last_group && springs[i+group] == '#' as u8 { continue; }
        // println!("{group_match} {end_match} {last_group} {unnaccounted}");
        // if returns 0 then break
        sum += if last_group { 1 } else { rec_fill(&springs[i+group+1..], &groups[1..]) };
    }
    // println!("Exit");
    return sum;
}

fn part2(filename: &str) -> i64 {
    let lines = file::read_to_lines(filename);
    let rows: Vec<(&str, Vec<i64>)> = lines.iter().map(|l| parse!(l, "{} {:,:}")).collect();
    
    let mut sum = 0;
    for (i, (springs, groups)) in rows.into_iter().enumerate() {
        println!("{i}");
        // unfold
        let spring_bytes: Vec<u8> = springs.bytes().collect();
        let mut springs_unfolded= spring_bytes.clone();
        let mut groups_unfolded: Vec<i64> = groups.clone();
        for _ in 0..4 { 
            springs_unfolded.push('?' as u8);
            springs_unfolded.extend(&spring_bytes);
            groups_unfolded.extend(&groups);
        }
        let i = rec_fill(&springs_unfolded, &groups_unfolded[..]);
        // println!("{i}");
        sum += i;
    }
    return sum;
}

// low 7064