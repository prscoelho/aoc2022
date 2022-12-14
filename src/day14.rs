use std::collections::{HashMap, HashSet};

use crate::runner::Solve;

pub struct Day14;

type Coordinate = (i32, i32);

fn parse_coordinate(input: &str) -> Coordinate {
    let mut it = input.split(',').filter_map(|s| s.parse().ok());

    (it.next().unwrap(), it.next().unwrap())
}

fn coords(from_coord: &Coordinate, to_coord: &Coordinate) -> Vec<Coordinate> {
    if from_coord.0 == to_coord.0 {
        let x = from_coord.0;
        let y0 = from_coord.1.min(to_coord.1);
        let y1 = from_coord.1.max(to_coord.1);

        (y0..=y1).map(|y| (x, y)).collect()
    } else {
        let y = from_coord.1;
        let x0 = from_coord.0.min(to_coord.0);
        let x1 = from_coord.0.max(to_coord.0);

        (x0..=x1).map(|x| (x, y)).collect()
    }
}

fn parse_line(input: &str) -> HashSet<Coordinate> {
    let coordinates: Vec<Coordinate> = input.split(" -> ").map(parse_coordinate).collect();

    coordinates
        .windows(2)
        .flat_map(|window| coords(&window[0], &window[1]))
        .collect::<HashSet<(i32, i32)>>()
}

fn parse_input(input: &str) -> HashSet<Coordinate> {
    input.lines().flat_map(parse_line).collect()
}

fn is_solid_without_ground(map: &HashMap<Coordinate, Tile>, at: &Coordinate, _max: i32) -> bool {
    map.get(at).is_some()
}

fn is_solid_with_ground(map: &HashMap<Coordinate, Tile>, at: &Coordinate, max: i32) -> bool {
    map.get(at).is_some() || at.1 == max + 2
}

fn produce(
    map: &HashMap<Coordinate, Tile>,
    at: Coordinate,
    max: i32,
    is_solid: impl Fn(&HashMap<Coordinate, Tile>, &Coordinate, i32) -> bool,
) -> Option<Coordinate> {
    let mut current = at;

    while !is_solid(map, &current, max) && current.1 <= max + 1 {
        let below = (current.0, current.1 + 1);

        if is_solid(map, &below, max) {
            let left = (below.0 - 1, below.1);
            let right = (below.0 + 1, below.1);

            if !is_solid(map, &left, max) {
                current = left;
            } else if !is_solid(map, &right, max) {
                current = right;
            } else {
                return Some(current);
            }
        } else {
            current = below;
        }
    }
    None
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
    Rock,
    Sand,
}

impl Solve<usize, usize> for Day14 {
    fn part1(input: &str) -> usize {
        let mut tiles: HashMap<Coordinate, Tile> = parse_input(input)
            .into_iter()
            .map(|coord| (coord, Tile::Rock))
            .collect();

        let source = (500, 0);
        let max_y = tiles.keys().map(|&(_, y)| y).max().unwrap();

        while let Some(coordinate) = produce(&tiles, source, max_y, is_solid_without_ground) {
            tiles.insert(coordinate, Tile::Sand);
        }

        tiles
            .into_iter()
            .filter(|&(_, tile)| tile == Tile::Sand)
            .count()
    }

    fn part2(input: &str) -> usize {
        let mut tiles: HashMap<Coordinate, Tile> = parse_input(input)
            .into_iter()
            .map(|coord| (coord, Tile::Rock))
            .collect();

        let source = (500, 0);
        let max_y = tiles.keys().map(|&(_, y)| y).max().unwrap();

        while let Some(coordinate) = produce(&tiles, source, max_y, is_solid_with_ground) {
            tiles.insert(coordinate, Tile::Sand);
        }

        tiles
            .into_iter()
            .filter(|&(_, tile)| tile == Tile::Sand)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

    #[test]
    fn example_p1() {
        let result = Day14::part1(EXAMPLE);
        let expected = 24;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day14::part2(EXAMPLE);
        let expected = 93;

        assert_eq!(result, expected);
    }
}
