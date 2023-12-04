use aoc_util::*;
use std::collections::HashMap;

pub fn main(filename: &str) {
    println!("Part 1:");
    let gears = part1(filename);
    println!("Part 2:");
    part2(gears);
}

#[derive(Clone, Copy)]
enum State {
    Idle,
    NumInv,
    NumVal,
}

fn part1(filename: &str) -> HashMap<(usize, usize), Vec<i32>> {
    let lines = file::read_to_lines(filename);
    let bytes: Vec<_> = lines.iter().map(|l| l.as_bytes().to_vec()).collect();
    let mut sum: i32 = 0;
    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for y in 0..bytes.len() {
        let mut state = State::Idle;
        let mut acc: i32 = 0;
        let mut gear_vec: Vec<(usize,usize)> = Vec::new();
        for x in 0..bytes[y].len() {
            match (state, (bytes[y][x] as char).is_digit(10)) {
                (State::Idle, true) => {
                    gear_vec.clear();
                    state = if is_part_num(&bytes, x, y, &mut gear_vec) { State::NumVal } else { State::NumInv };
                    acc = bytes[y][x] as i32 - '0' as i32;
                },
                (State::NumInv, true) => {
                    if is_part_num(&bytes, x, y, &mut gear_vec) { state = State::NumVal; }
                    acc = acc*10 + bytes[y][x] as i32 - '0' as i32;
                },
                (State::NumVal, true) => acc = acc*10 + bytes[y][x] as i32 - '0' as i32,
                (_, false) => {
                    if let State::NumVal = state {
                        sum += acc;
                        for g in &gear_vec {
                            if !gears.contains_key(g) {
                                gears.insert(*g, Vec::new());
                            }
                            gears.get_mut(g).unwrap().push(acc);
                        }
                    }
                    state = State::Idle;
                }
            }
        }
        if let State::NumVal = state {
            sum += acc;
            for g in &gear_vec {
                if !gears.contains_key(g) {
                    gears.insert(*g, Vec::new());
                }
                gears.get_mut(g).unwrap().push(acc);
            }
        }
    }

    println!("Sum is {sum}");
    return gears;
}

fn part2(gears: HashMap<(usize, usize), Vec<i32>>) {
    let mut sum: i32 = 0;
    for (c, v) in gears {
        println!("{:?} : {:?}", c, v);
        if v.len() == 2 {
            sum += v[0]*v[1];
        }
    }
    println!("Sum is {sum}");
}

fn is_part_num(bytes: &Vec<Vec<u8>>, x: usize, y: usize, gears: &mut Vec<(usize,usize)>) -> bool {
    use std::cmp::{min, max};
    let mut adj = false;
    for yr in (max(y, 1)-1)..=(min(y,bytes.len()-2)+1) {
        for xr in (max(x, 1)-1)..=(min(x,bytes[y].len()-2)+1) {
            if is_symbol(bytes[yr][xr] as char) {
                if bytes[yr][xr] as char == '*' {
                    gears.push((xr, yr));
                }
                adj = true;
            }
        }
    }
    return adj;
}

fn is_symbol(c: char) -> bool {
    return !c.is_digit(10) && c != '.';
}