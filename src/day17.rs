use std::collections::{BTreeSet, HashMap};

use crate::runner::Solve;

pub struct Day17;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Horizontal,
    Cross,
    LShape,
    Vertical,
    Square,
}

use Tile::*;

// y, x
type Coordinate = (i32, i32);

impl Tile {
    fn coords(&self) -> Vec<Coordinate> {
        match self {
            Horizontal => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Cross => vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
            LShape => vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
            Vertical => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Square => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        }
    }

    fn placed_at(&self, start: Coordinate) -> Vec<Coordinate> {
        self.coords().into_iter().map(|c| add(c, start)).collect()
    }
}

fn add(c1: Coordinate, c2: Coordinate) -> Coordinate {
    (c1.0 + c2.0, c1.1 + c2.1)
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Jet {
    Left,
    Right,
}

impl Jet {
    fn dir(&self) -> Coordinate {
        match self {
            Jet::Left => (0, -1),
            Jet::Right => (0, 1),
        }
    }
}

fn within_bounds(coordinate: Coordinate) -> bool {
    coordinate.0 >= 0 && coordinate.1 >= 0 && coordinate.1 < 7
}

fn can_move_dir(rock_coords: &[Coordinate], dir: Coordinate, set: &BTreeSet<Coordinate>) -> bool {
    rock_coords
        .iter()
        .map(|&c| add(c, dir))
        .all(|c| within_bounds(c) && !set.contains(&c))
}

fn snapshot_line(set: &BTreeSet<Coordinate>, y: i32) -> u8 {
    let mut result = 0;

    if y < 0 {
        return 0x7F;
    }

    for x in 0..7 {
        if set.contains(&(y, x)) {
            result += 1 << x;
        }
    }

    result
}

fn snapshot(set: &BTreeSet<Coordinate>, y: i32) -> Option<[u8; 4]> {
    let result = [
        snapshot_line(set, y - 3),
        snapshot_line(set, y - 2),
        snapshot_line(set, y - 1),
        snapshot_line(set, y),
    ];

    if result.iter().any(|u| *u == 0x7F) {
        Some(result)
    } else {
        None
    }
}

fn height_after(rock_amount: usize, jets: Vec<Jet>) -> usize {
    let mut set: BTreeSet<(i32, i32)> = BTreeSet::new();

    let mut jets = jets.into_iter().enumerate().cycle().peekable();

    let mut snapshots: HashMap<([u8; 4], usize, usize), (i32, usize)> = HashMap::new();
    let mut history = Vec::new();

    for (current_rock, (rock_idx, rock)) in ORDER
        .iter()
        .enumerate()
        .cycle()
        .take(rock_amount)
        .enumerate()
    {
        let current_height = set.last().map(|c| c.0).unwrap_or(-1);
        history.push(current_height);

        if let Some(snap) = snapshot(&set, current_height) {
            let jet_idx = jets.peek().unwrap().0;
            let key = (snap, rock_idx, jet_idx);
            if snapshots.contains_key(&key) {
                let &(cycle_start_height, cycle_start_rocks) = snapshots.get(&key).unwrap();

                let cycle_height = (current_height - cycle_start_height) as usize;
                let cycle_rocks = current_rock - cycle_start_rocks;

                let amount_required = rock_amount - cycle_start_rocks;

                let cycle_count = amount_required / cycle_rocks;
                let remaining_rocks = amount_required % cycle_rocks;

                let total_by_cycle = cycle_height * cycle_count;

                let remaining_height = history[cycle_start_rocks + remaining_rocks] as usize
                    - history[cycle_start_rocks] as usize;

                return total_by_cycle + remaining_height + cycle_start_height as usize + 1;
            }

            snapshots.insert(key, (current_height, current_rock));
        }

        let rock_y = current_height + 4;
        let rock_x = 2;

        let mut rock_coords = rock.placed_at((rock_y, rock_x));
        loop {
            let jet = jets.next().unwrap().1;
            let dir = jet.dir();

            if can_move_dir(&rock_coords, dir, &set) {
                rock_coords.iter_mut().for_each(|c| *c = add(*c, dir));
            }

            if can_move_dir(&rock_coords, (-1, 0), &set) {
                rock_coords.iter_mut().for_each(|c| *c = add(*c, (-1, 0)));
            } else {
                break;
            }
        }

        set.extend(rock_coords);
    }

    set.last().unwrap().0 as usize + 1
}

fn parse_jet(c: char) -> Jet {
    match c {
        '<' => Jet::Left,
        '>' => Jet::Right,
        _ => panic!("Unexpected jet char"),
    }
}

fn parse_input(input: &str) -> Vec<Jet> {
    input.trim().chars().map(parse_jet).collect()
}

const ORDER: [Tile; 5] = [Horizontal, Cross, LShape, Vertical, Square];

impl Solve<usize, usize> for Day17 {
    fn part1(input: &str) -> usize {
        let jets = parse_input(input);
        height_after(2022, jets)
    }
    fn part2(input: &str) -> usize {
        let jets = parse_input(input);

        height_after(1000000000000, jets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;

    #[test]
    fn example_p1() {
        let result = Day17::part1(EXAMPLE);
        let expected = 3068;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day17::part2(EXAMPLE);
        let expected = 1514285714288;

        assert_eq!(result, expected);
    }
}
