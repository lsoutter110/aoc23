mod day1;
mod day2;

fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    
    let day = match args.next() {
        Some(s) => s,
        None => { println!("Argument error: Expected day argument (1-25)!"); return },
    };
    let file = match args.next() {
        Some(s) => s,
        None => { println!("Argument error: Expected filename argument"); return },
    };

    match &day[..] {
        "1" => day1::main(&file[..]),
        "2" => day2::main(&file[..]),
        d => println!("Argument error: Unknown day '{d}'"),
    }
}