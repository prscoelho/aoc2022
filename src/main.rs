use std::env;

use crate::runner::{parse_day, read_day_input, Solve};
mod runner;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;

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
        3 => day03::Day03::solve(&input),
        4 => day04::Day04::solve(&input),
        5 => day05::Day05::solve(&input),
        6 => day06::Day06::solve(&input),
        7 => day07::Day07::solve(&input),
        8 => day08::Day08::solve(&input),
        9 => day09::Day09::solve(&input),
        10 => day10::Day10::solve(&input),
        11 => day11::Day11::solve(&input),
        12 => day12::Day12::solve(&input),
        13 => day13::Day13::solve(&input),
        14 => day14::Day14::solve(&input),
        15 => day15::Day15::solve(&input),
        16 => day16::Day16::solve(&input),
        17 => day17::Day17::solve(&input),
        18 => day18::Day18::solve(&input),
        19 => day19::Day19::solve(&input),
        20 => day20::Day20::solve(&input),
        21 => day21::Day21::solve(&input),
        _ => {
            eprintln!("Not implemented yet");
            return;
        }
    };

    println!("Running day: {:02}", day);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
