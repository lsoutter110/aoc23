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

fn part1(filename: &str) {
    let file_string = read_to_lines(filename);

    let sum: u32 = file_string.iter().map(|line| {
        let first = line.chars().fold(None, |s, c| match (s, c) {
            (Some(n), _) => Some(n),
            (None, n @ '0'..='9') => Some(n.to_digit(10).unwrap()),
            _ => None,
        }).unwrap();
        let last = line.chars().rev().fold(None, |s, c| match (s, c) {
            (Some(n), _) => Some(n),
            (None, n @ '0'..='9') => Some(n.to_digit(10).unwrap()),
            _ => None,
        }).unwrap();
        return 10*first + last;
    }).sum();

    println!("Sum of values = {sum}");
}

fn part2(filename: &str) {
    let digit_strs = vec![(0,"zero"), (1,"one"), (2,"two"), (3,"three"), (4,"four"), (5,"five"), (6,"six"), (7,"seven"), (8,"eight"), (9,"nine")];
    let file_string = read_to_lines(filename);

    // //screen for strings
    // for line in &mut file_string {
        
    //     for line_i in 0..line.len() {
    //         for di in 0..digit_strs.len() {
    //             let dlen = digit_strs[di].len();
    //             if &line[line_i..std::cmp::min(line_i+dlen, line.len())] == digit_strs[di] {
    //                 line.replace_range(line_i..=line_i, &format!("{di}"));
    //             }
    //         }
    //     }
    // }

    let sum: u32 = file_string.iter()
        .map(|line| {
            let line = digit_strs.iter().fold(line.to_string(), |l, (i, n)| l.replace(n, &format!("{n}{i}{n}")));
            // println!("{}", line);
            let first = line.chars().fold(None, |s, c| match (s, c) {
                (Some(n), _) => Some(n),
                (None, n @ '0'..='9') => Some(n.to_digit(10).unwrap()),
                _ => None,
            }).unwrap();
            let last = line.chars().rev().fold(None, |s, c| match (s, c) {
                (Some(n), _) => Some(n),
                (None, n @ '0'..='9') => Some(n.to_digit(10).unwrap()),
                _ => None,
            }).unwrap();

            return 10*first + last;
        })
        .sum();

    println!("Sum of values = {sum}");
}

fn read_to_lines(filename: &str) -> Vec<String> {
    use std::{fs::File, path::Path, io::Read};
    let path = Path::new(filename);

    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("Couldn't open {}: {}", filename, e),
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => (),
        Err(e) => panic!("Couldn't read {} to string: {}", filename, e),
    }

    return s.split("\n").map(|line| line.to_string()).collect();
}