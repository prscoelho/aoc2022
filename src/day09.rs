use std::collections::HashSet;

use crate::runner::Solve;

pub struct Day09;

#[derive(Clone, Copy, Debug)]
enum Motion {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl Motion {
    fn dir(&self) -> (i32, i32) {
        match self {
            Motion::Up(_) => (0, 1),
            Motion::Down(_) => (0, -1),
            Motion::Left(_) => (-1, 0),
            Motion::Right(_) => (1, 0),
        }
    }

    fn value(self) -> i32 {
        match self {
            Motion::Up(u) => u,
            Motion::Down(d) => d,
            Motion::Left(l) => l,
            Motion::Right(r) => r,
        }
    }

    fn apply_once(&self, (x, y): (i32, i32)) -> (i32, i32) {
        let (dx, dy) = self.dir();
        (x + dx, y + dy)
    }
}

fn parse_line(input: &str) -> Motion {
    let (left, right) = input
        .split_once(' ')
        .expect("expected a motion -- two words");
    let value = right.parse().unwrap();

    match left {
        "U" => Motion::Up(value),
        "D" => Motion::Down(value),
        "L" => Motion::Left(value),
        "R" => Motion::Right(value),
        _ => panic!("expected a motion value [UDLR]"),
    }
}

fn parse_input(input: &str) -> Vec<Motion> {
    input.lines().map(parse_line).collect()
}

/// Distance between two points but you can only move straight
fn manhattan_distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

/// Distance from two points, assuming you can move diagonally
fn distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs().max((y1 - y2).abs())
}

fn neighbours((x, y): (i32, i32)) -> Vec<(i32, i32)> {
    let mut result = Vec::with_capacity(8);
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == dy && dx == 0 {
                continue;
            }

            result.push((x + dx, y + dy));
        }
    }

    result
}

fn best_neighbour(head: (i32, i32), position1: (i32, i32), position2: (i32, i32)) -> (i32, i32) {
    if manhattan_distance(position1, head) < manhattan_distance(position2, head) {
        position1
    } else {
        position2
    }
}

fn walk_rope(motions: Vec<Motion>, n: usize) -> HashSet<(i32, i32)> {
    let mut result = HashSet::new();

    let mut knots = vec![(0, 0); n];

    for motion in motions {
        for _ in 0..motion.value() {
            knots[0] = motion.apply_once(knots[0]);

            for idx in 1..knots.len() {
                let knot_ahead = knots[idx - 1];
                let knot_behind = knots[idx];

                if distance(knot_ahead, knot_behind) > 1 {
                    let closest_position = neighbours(knot_behind)
                        .into_iter()
                        .reduce(|best, item| best_neighbour(knot_ahead, best, item))
                        .unwrap();
                    knots[idx] = closest_position;
                }
            }

            result.insert(*knots.last().unwrap());
        }
    }

    result
}

impl Solve<usize, usize> for Day09 {
    fn part1(input: &str) -> usize {
        let motions = parse_input(input);

        walk_rope(motions, 2).len()
    }

    fn part2(input: &str) -> usize {
        let motions = parse_input(input);

        walk_rope(motions, 10).len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

    const EXAMPLE2: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

    #[test]
    fn example_p1() {
        let result = Day09::part1(EXAMPLE);
        let expected = 13;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day09::part2(EXAMPLE2);
        let expected = 36;

        assert_eq!(result, expected);
    }
}
