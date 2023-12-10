use aoc_util::{*, cspace::{Vec2, Coord2}};
use std::time::Instant;
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
    let plane = file::read_to_char_plane(filename);

    let mut pos = Vec2::new(0, 0);
    for y in 0..plane.height {
        for x in 0..plane.width {
            if plane[(x,y)] == 'S' {
                pos = Vec2::new(x, y);
            }
        }
    }

    let mut last = pos;
    pos = match (plane.get(pos.up()), plane.get(pos.down()), plane.get(pos.left()), plane.get(pos.right())) {
        (Some('|'|'7'|'F'),_,_,_) => pos.up(),
        (_,Some('|'|'J'|'L'),_,_) => pos.down(),
        (_,_,Some('-'|'F'|'L'),_) => pos.left(),
        (_,_,_,Some('-'|'J'|'7')) => pos.right(),
        _ => panic!("No starting pipe found!"),
    };

    let mut steps = 1;
    loop {
        steps += 1;
        let new = match plane[pos] {
            '|' => if last == pos.up() { pos.down() } else { pos.up() },
            '-' => if last == pos.left() { pos.right() } else { pos.left() },
            'L' => if last == pos.up() { pos.right() } else { pos.up() },
            'J' => if last == pos.up() { pos.left() } else { pos.up() },
            '7' => if last == pos.down() { pos.left() } else { pos.down() },
            'F' => if last == pos.down() { pos.right() } else { pos.down() },
            '.' => panic!("Ran onto ground!"),
            'S' => break,
            _ => panic!("Unknown tile!"),
        };
        last = pos;
        pos = new;
    }
    return steps / 2;
}

fn part2(filename: &str) -> i64 {
    let mut plane = file::read_to_char_plane(filename);

    let mut pos = Vec2::new(0, 0);
    for y in 0..plane.height {
        for x in 0..plane.width {
            if plane[(x,y)] == 'S' {
                pos = Vec2::new(x, y);
            }
        }
    }

    let mut last = pos;
    let connections = (
        match plane.get(pos.up()) { Some('|'|'7'|'F') => true, _ => false },
        match plane.get(pos.right()) { Some('-'|'J'|'7') => true, _ => false },
        match plane.get(pos.down()) { Some('|'|'J'|'L') => true, _ => false },
        match plane.get(pos.left()) { Some('-'|'F'|'L') => true, _ => false },
    );
    pos = match connections {
        (true,_,_,_) => pos.up(),
        (_,true,_,_) => pos.right(),
        (_,_,true,_) => pos.down(),
        (_,_,_,true) => pos.left(),
        _ => panic!("No starting pipe found!"),
    };
    plane[last] = match connections {
        (true,true,_,_) => 'L',
        (_,true,true,_) => 'F',
        (_,_,true,true) => '7',
        (true,_,_,true) => 'J',
        (true,_,true,_) => 'S',
        (_,true,_,true) => 's',
        _ => panic!("Unrecognised start!"),
    };

    //replace pipe with 'S's
    loop {
        let new = match plane[pos] {
            '|' => if last == pos.up() { pos.down() } else { pos.up() },
            '-' => if last == pos.left() { pos.right() } else { pos.left() },
            'L' => if last == pos.up() { pos.right() } else { pos.up() },
            'J' => if last == pos.up() { pos.left() } else { pos.up() },
            '7' => if last == pos.down() { pos.left() } else { pos.down() },
            'F' => if last == pos.down() { pos.right() } else { pos.down() },
            '.' => panic!("Ran onto ground!"),
            'S'|'s' => break,
            _ => panic!("Unknown tile!"),
        };
        if let 'L'|'J'|'|' = plane[pos]{
            plane[pos] = 'S'; //replace verticals
        } else {
            plane[pos] = 's';
        }
        last = pos;
        pos = new;
    }
    
    let mut count = 0;
    for y in 0..plane.height {
        let mut inside = false;
        for x in 0..plane.width {
            if plane[(x,y)] == 'S' {
                inside = !inside;
            } else if inside && plane[(x,y)] != 's' {
                count += 1;
            }
        }
    }
    return count;
}
