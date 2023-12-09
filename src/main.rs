#![allow(unused_imports)]
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let mut args = std::env::args();
    let _ = args.next();

    let day = match args.next() {
        Some(s) => s,
        None => { println!("Argument error: Expected day argument (1-25)!"); return },
    };

    let test = day.chars().last() == Some('t');
    let day_end = if test { day.len()-1 } else { day.len() };

    let file = match args.next() {
        Some(s) => s,
        None => format!("input/day{}/{}.txt", &day[..day_end], if test { "test" } else { "input" }),
    };

    match &day[..day_end] {
        "1" => day1::main(&file[..]),
        "2" => day2::main(&file[..]),
        "3" => day3::main(&file[..]),
        "4" => day4::main(&file[..]),
        "5" => day5::main(&file[..]),
        "6" => day6::main(&file[..]),
        "7" => day7::main(&file[..]),
        "8" => day8::main(&file[..]),
        "9" => day9::main(&file[..]),
        d => println!("Argument error: Unknown day '{d}'"),
    }
}
