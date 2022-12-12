use std::collections::{HashMap, HashSet, VecDeque};

use crate::runner::Solve;

pub struct Day12;

fn height(ch: char) -> u8 {
    match ch {
        'S' => height('a'),
        'E' => height('z'),
        'a'..='z' => (ch as u32 - 'a' as u32) as u8,
        _ => panic!("unexpected ch in height calculation"),
    }
}

fn parse_heightmap(input: &str) -> HashMap<(i32, i32), char> {
    let mut result = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            result.insert((col as i32, row as i32), ch);
        }
    }

    result
}

fn find_ch(heightmap: &HashMap<(i32, i32), char>, ch: char) -> (i32, i32) {
    *heightmap.iter().find(|(_, v)| **v == ch).unwrap().0
}

fn neighbours((col, row): (i32, i32)) -> [(i32, i32); 4] {
    [
        (col + 1, row),
        (col - 1, row),
        (col, row + 1),
        (col, row - 1),
    ]
}

fn uphill_condition(current_height: u8, next_height: u8) -> bool {
    next_height.saturating_sub(1) <= current_height
}

// reversed uphill
fn downhill_condition(current_height: u8, next_height: u8) -> bool {
    uphill_condition(next_height, current_height)
}

fn bfs(
    start: (i32, i32),
    goal: char,
    heightmap: &HashMap<(i32, i32), char>,
    condition: impl Fn(u8, u8) -> bool,
) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_front((0, start));
    visited.insert(start);

    while let Some((steps, position)) = queue.pop_front() {
        let current_ch = *heightmap.get(&position).unwrap();
        if current_ch == goal {
            return steps;
        }

        let current_height = height(current_ch);

        for neighbour in neighbours(position) {
            if let Some(&next_ch) = heightmap.get(&neighbour) {
                let next_height = height(next_ch);

                if condition(current_height, next_height) && visited.insert(neighbour) {
                    queue.push_back((steps + 1, neighbour));
                }
            }
        }
    }
    usize::MAX
}

impl Solve<usize, usize> for Day12 {
    fn part1(input: &str) -> usize {
        let height_map = parse_heightmap(input);
        let start = find_ch(&height_map, 'S');

        bfs(start, 'E', &height_map, uphill_condition)
    }
    fn part2(input: &str) -> usize {
        let height_map = parse_heightmap(input);
        let end = find_ch(&height_map, 'E');

        bfs(end, 'a', &height_map, downhill_condition)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

    #[test]
    fn example_p1() {
        let result = Day12::part1(EXAMPLE);
        let expected = 31;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day12::part2(EXAMPLE);
        let expected = 29;

        assert_eq!(result, expected);
    }
}
