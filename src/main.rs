use std::env;

use crate::runner::{parse_day, read_day_input, Solve};
mod runner;

mod day01;
mod day02;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("error: missing day to run.");
        return;
    }

    let day = match parse_day(&args[1]) {
        Ok(day) => day,
        Err(e) => {
            eprintln!("Failed to parse day: {}", e);
            return;
        }
    };
    let input = read_day_input(day);

    let (p1, p2) = match day {
        1 => day01::Day01::solve(&input),
        2 => day02::Day02::solve(&input),
        _ => {
            eprintln!("Not implemented yet");
            return;
        }
    };

    println!("Running day: {:02}", day);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
