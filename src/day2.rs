use aoc_util::{file, string};
use std::collections::HashMap;

pub fn main(filename: &str) {
    println!("Part 1:");
    part1(filename);
    println!("Part 2:");
    part2(filename);
}

fn part1(filename: &str) {
    let lines = file::read_to_lines(filename);
    let mut valid_games: HashMap<u32, bool> = HashMap::new();
    let targets = vec![12, 13, 14];

    for line in lines {
        let mut line_split_iter = line.split(": ");
        let game_id = string::atoi(line_split_iter.next().unwrap().split(" ").collect::<Vec<_>>()[1], 10);
        let game_data = line_split_iter.next().unwrap().split("; ");
        
        if !valid_games.contains_key(&game_id) {
            valid_games.insert(game_id, true);
        }

        for d in game_data {
            for cube in d.split(", ") {
                let mut cube_iter = cube.split(" ");
                let num = string::atoi(cube_iter.next().unwrap(), 10);
                let col = match cube_iter.next().unwrap() {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    c => panic!("Unexpected colour {c}!"),
                };
                if num > targets[col] {
                    *valid_games.get_mut(&game_id).unwrap() = false;
                }
            }
        }

    }

    let mut sum: u32 = 0;
    for (game, valid) in valid_games {
        if valid {
            sum += game;
        }
    }
    println!("Sum is {sum}");
}

fn part2(filename: &str) {
    let lines = file::read_to_lines(filename);
    let mut valid_games: HashMap<u32, Vec<u32>> = HashMap::new();

    for line in lines {
        let mut line_split_iter = line.split(": ");
        let game_id = string::atoi(line_split_iter.next().unwrap().split(" ").collect::<Vec<_>>()[1], 10);
        let game_data = line_split_iter.next().unwrap().split("; ");
        
        if !valid_games.contains_key(&game_id) {
            valid_games.insert(game_id, vec![0, 0, 0]);
        }

        let cur_game = valid_games.get_mut(&game_id).unwrap();
        for d in game_data {
            for cube in d.split(", ") {
                let mut cube_iter = cube.split(" ");
                let num = string::atoi(cube_iter.next().unwrap(), 10);
                let col = match cube_iter.next().unwrap() {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    c => panic!("Unexpected colour {c}!"),
                };
                if num > cur_game[col] {
                    cur_game[col] = num;
                }
            }
        }

    }

    let mut sum: u32 = 0;
    for (_, nums) in valid_games {
        sum += nums.iter().fold(1, |m, i| m*i);
    }
    println!("Sum is {sum}");
}