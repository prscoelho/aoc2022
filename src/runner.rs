use std::fmt::Display;
use std::num::ParseIntError;
use std::{fs::File, io::Read};

pub trait Solve<T1, T2>
where
    T1: ToString,
    T2: ToString,
{
    fn part1(input: &str) -> T1;
    fn part2(input: &str) -> T2;

    fn solve(input: &str) -> (String, String) {
        (
            Self::part1(input).to_string(),
            Self::part2(input).to_string(),
        )
    }
}

#[derive(Debug, Clone)]
pub enum DayError {
    NotADay,
    NotANumber(ParseIntError),
}

impl Display for DayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DayError::NotADay => writeln!(
                f,
                "number is not an advent of code day, should be between 1-25"
            ),
            DayError::NotANumber(e) => writeln!(f, "not a number: {}", e),
        }
    }
}

pub fn parse_day(day_str: &str) -> Result<i32, DayError> {
    let parsed: Result<i32, _> = day_str.parse();
    match parsed {
        Ok(num) => {
            if (1..=25).contains(&num) {
                Ok(num)
            } else {
                Err(DayError::NotADay)
            }
        }
        Err(e) => Err(DayError::NotANumber(e)),
    }
}

pub fn read_day_input(day: i32) -> String {
    let filename = format!("./input/{:02}.input", day);
    let mut file = File::open(&filename)
        .unwrap_or_else(|_| panic!("failed to open \"{}\", does it exist?", filename));
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
