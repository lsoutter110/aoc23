use aoc_util::{*, cspace::Plane};
use std::{time::Instant, collections::HashMap};
use prse::{self, Parse, parse, try_parse};
use std::cmp::min;

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
    let mut fields = Vec::new();
    let mut cplane = Plane::new();
    for l in file::read_to_lines(filename) {
        if l.len() == 0 {
            fields.push(cplane);
            cplane = Plane::new();
            continue;
        }
        cplane.push_row(l.chars().collect());
    }
    fields.push(cplane);

    return fields.iter()
        .map(find_reflections)
        .map(|(c,r)| c + 100*r)
        .sum::<usize>() as i64;
}

fn part2(filename: &str) -> i64 {
    let mut fields = Vec::new();
    let mut cplane = Plane::new();
    for l in file::read_to_lines(filename) {
        if l.len() == 0 {
            fields.push(cplane);
            cplane = Plane::new();
            continue;
        }
        cplane.push_row(l.chars().collect());
    }
    fields.push(cplane);

    return fields.iter()
        .map(find_smudged)
        .map(|(c,r)| c + 100*r)
        .sum::<usize>() as i64;
}

fn find_reflections(plane: &Plane<char>) -> (usize, usize) {
    let cols = match check_sym_h(plane, 0).get(0) {
        Some(p) => p + 1,
        None => 0,
    }; // columns left of pivot
    
    let rows = match check_sym_v(plane, 0).get(0) {
        Some(p) => p + 1,
        None => 0,
    }; // rows above pivot

    return (cols, rows);
}

fn find_smudged(plane: &Plane<char>) -> (usize, usize) {
    let new_sym = check_sym_h(plane, 1);
    let cols = match match check_sym_h(plane, 0).get(0) {
        Some(old_sym) => new_sym.iter().filter(|&x| x != old_sym).next(),
        None => new_sym.get(0),
    } {
        Some(p) => p + 1,
        None => 0,
    };
    
    let new_sym = check_sym_v(plane, 1);
    let rows = match match check_sym_v(plane, 0).get(0) {
        Some(old_sym) => new_sym.iter().filter(|&x| x != old_sym).next(),
        None => new_sym.get(0),
    } {
        Some(p) => p + 1,
        None => 0,
    };
    // println!("Cols = {cols}, Rows = {rows}");

    return (cols, rows);
}

fn check_sym_h(plane: &Plane<char>, errors: usize) -> Vec<usize> {
    let mut pivots: Vec<usize> = (0..plane.width-1).collect();
    let mut errors: HashMap<usize, usize> = pivots.iter()
        .fold(HashMap::new(), |mut h,p| { h.insert(*p, errors); h });
    // println!("init {:?}", errors);
    for row in 0..plane.height {
        pivots = pivots.into_iter().filter(|pivot|
            (0..=min(*pivot, plane.width-2-pivot))
                .all(|i| {
                    if plane[[pivot-i, row]] != plane[[pivot+i+1, row]] {
                        if errors[pivot] == 0 { return false; }
                        *errors.get_mut(pivot).unwrap() -= 1;
                    }
                    return true;
                })
        ).collect();
    }
    // println!("| {:?}", pivots);
    return pivots;
}

fn check_sym_v(plane: &Plane<char>, errors: usize) -> Vec<usize> {
    let mut pivots: Vec<usize> = (0..plane.height-1).collect();
    let mut errors: HashMap<usize, usize> = pivots.iter()
        .fold(HashMap::new(), |mut h,p| { h.insert(*p, errors); h });
    // println!("init {:?}", errors);
    for col in 0..plane.width {
        pivots = pivots.into_iter().filter(|pivot|
            (0..=min(*pivot, plane.height-2-pivot))
                .all(|i| {
                    if plane[[col, pivot-i]] != plane[[col, pivot+i+1]] {
                        if errors[pivot] == 0 { return false; }
                        *errors.get_mut(pivot).unwrap() -= 1;
                    }
                    return true;
                })
        ).collect();
    }
    // println!("- {:?} {:?}", pivots, errors);
    return pivots;
}