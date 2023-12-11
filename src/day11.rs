use aoc_util::*;
use std::{time::Instant, collections::HashMap};
use prse::{self, Parse, parse, try_parse};
use aoc_util::cspace::Vec2;

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
    let mut plane = file::read_to_char_plane(filename);
    //insert space
    let mut row = 0;
    while row < plane.height {
        if (0..plane.width).all(|x| plane[[x, row]] == '.') {
            row += 1;
            let mut new = Vec::new();
            for _ in 0..plane.width { new.push('.'); }
            plane.insert_row(row, new);
        }
        row += 1;
    }
    let mut col = 0;
    while col < plane.width {
        if (0..plane.height).all(|y| plane[[col, y]] == '.') {
            col += 1;
            let mut new = Vec::new();
            for _ in 0..plane.height { new.push('.'); }
            plane.insert_col(col, new);
        }
        col += 1;
    }
    
    let mut galaxies = Vec::new();
    for y in 0..plane.height {
        for x in 0..plane.width {
            print!("{}", plane[[x,y]]);
            if plane[[x,y]] == '#' {
                galaxies.push(Vec2::new(x, y));
            }
        }
        println!("");
    }

    let mut sum = 0;
    for a in &galaxies {
        for b in &galaxies {
            sum += (i32::abs(a.x-b.x)+i32::abs(a.y-b.y)) as i64;
        }
    }
    return sum/2;
}

fn part2(filename: &str) -> i64 {
    let mut plane = file::read_to_char_plane(filename);
    //insert space
    let mut rowr: i64 = 0;
    let mut row_index = Vec::new();
    for row in 0..plane.height {
        row_index.push(rowr);
        if (0..plane.width).all(|x| plane[[x, row]] == '.') {
            rowr += 999999;
        }
        rowr += 1;
    }
    let mut colr: i64 = 0;
    let mut col_index = Vec::new();
    for col in 0..plane.width {
        col_index.push(colr);
        if (0..plane.height).all(|y| plane[[col, y]] == '.') {
            colr += 999999;
        }
        colr += 1;
    }
    
    let mut galaxies = Vec::new();
    for y in 0..plane.height {
        for x in 0..plane.width {
            print!("{}", plane[[x,y]]);
            if plane[[x,y]] == '#' {
                galaxies.push(Vec2::new(x, y));
            }
        }
        println!("");
    }

    let mut sum = 0;
    for a in &galaxies {
        for b in &galaxies {
            sum += i64::abs(col_index[a.x as usize]-col_index[b.x as usize])
                +  i64::abs(row_index[a.y as usize]-row_index[b.y as usize]);
        }
    }
    return sum/2;
}
