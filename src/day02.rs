use crate::runner::Solve;

pub struct Day02;

fn read_input(input: &str) -> Vec<(&str, &str)> {
    let mut result = Vec::new();
    for line in input.lines() {
        let tuple = line.split_once(' ').unwrap();
        result.push(tuple);
    }
    result
}

#[derive(Clone, Copy, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

fn decode_left(s: &str) -> Shape {
    match s {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("Unexpected shape value {}", s),
    }
}

fn decode_right(s: &str) -> Shape {
    match s {
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,
        _ => panic!("unexpected shape value {}", s),
    }
}

#[derive(Clone, Copy, Debug,PartialEq, Eq)]
enum Outcome {
    Loss,
    Win,
    Draw
}

fn round_outcome(left: Shape, right: Shape) -> Outcome {
    match left {
        Shape::Rock => {
            match right {
                Shape::Rock => Outcome::Draw,
                Shape::Paper => Outcome::Win,
                Shape::Scissors => Outcome::Loss,
            }
        },
        Shape::Paper => {
            match right {
                Shape::Rock => Outcome::Loss,
                Shape::Paper => Outcome::Draw,
                Shape::Scissors => Outcome::Win,
            }
        },
        Shape::Scissors => {
            match right {
                Shape::Rock => Outcome::Win,
                Shape::Paper => Outcome::Loss,
                Shape::Scissors => Outcome::Draw,
            }
        },
    }
}

fn score(shape: Shape, outcome: Outcome) -> u32 {
    let from_shape = match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };
    let from_outcome = match outcome {
        Outcome::Loss => 0,
        Outcome::Win => 6,
        Outcome::Draw => 3,
    };

    from_shape + from_outcome
}

fn decode_outcome(s: &str) -> Outcome {
    match s {
        "X" => Outcome::Loss,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("unexpected shape value {}", s),
    }
}

fn shape_needed(left: Shape, outcome: Outcome) -> Shape {
    for right in [Shape::Rock, Shape::Paper, Shape::Scissors] {
        if round_outcome(left, right) == outcome {
            return right;
        }
    }
    panic!("Could not find a shape that resulted in expected outcome, this should not happen");
}

impl Solve<u32, u32> for Day02 {
    fn part1(input: &str) -> u32 {
        let rounds = read_input(input);
        let mut result = 0;
        for (left, right) in rounds {
            let left_shape = decode_left(left);
            let right_shape = decode_right(right);

            let outcome = round_outcome(left_shape, right_shape);
            
            result += score(right_shape, outcome);
        }

        result
    }
    fn part2(input: &str) -> u32 {
        let rounds = read_input(input);
        let mut result = 0;
        for (left, right) in rounds {
            let left_shape = decode_left(left);
            let required_outcome = decode_outcome(right);

            let right_shape = shape_needed(left_shape, required_outcome);

            result += score(right_shape, required_outcome);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"A Y
B X
C Z"#;

    #[test]
    fn parses_example() {
        let tuples = read_input(EXAMPLE);
        assert_eq!(tuples.len(), 3);
        assert_eq!(tuples[0], ("A", "Y"));
        assert_eq!(tuples[1], ("B", "X"));
        assert_eq!(tuples[2], ("C", "Z"));
    }

    #[test]
    fn example_p1() {
        let result = Day02::part1(EXAMPLE);
        let expected = 15;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day02::part2(EXAMPLE);
        let expected = 12;

        assert_eq!(result, expected);
    }
}
