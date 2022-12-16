use crate::runner::Solve;

pub struct Day15;

type Coordinate = (i32, i32);

fn parse_coordinate_at_end(input: &str) -> Coordinate {
    let coordinate_str = input.split("at ").nth(1).unwrap();

    let mut it = coordinate_str
        .split(", ")
        .filter_map(|text| text[2..].parse().ok());
    (it.next().unwrap(), it.next().unwrap())
}

fn parse_line(input: &str) -> (Coordinate, Coordinate) {
    let mut it = input.split(": ").map(parse_coordinate_at_end);
    (it.next().unwrap(), it.next().unwrap())
}

fn manhattan_distance(coord1: &Coordinate, coord2: &Coordinate) -> usize {
    ((coord1.0 - coord2.0).abs() + (coord1.1 - coord2.1).abs()) as usize
}

fn ranges_at_row(scanner: Coordinate, beacon: Coordinate, row: i32) -> Option<(i32, i32)> {
    let distance = manhattan_distance(&scanner, &beacon);

    // can it reach the row?
    let closest_col_to_row = (scanner.0, row);
    let distance_to_row = manhattan_distance(&scanner, &closest_col_to_row);

    if distance_to_row <= distance {
        let diff = (distance - distance_to_row) as i32;
        Some((scanner.0 - diff, scanner.0 + diff))
    } else {
        None
    }
}

fn try_merge(range1: (i32, i32), range2: (i32, i32)) -> Option<(i32, i32)> {
    if range2.0 <= range1.1 + 1 {
        Some((range1.0, range1.1.max(range2.1)))
    } else {
        None
    }
}

fn merge_ranges(mut ranges: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    ranges.sort();
    let mut result = Vec::new();
    let mut idx = 0;
    while idx < ranges.len() {
        let mut looking_at = idx + 1;
        let mut current = ranges[idx];

        loop {
            if looking_at >= ranges.len() {
                break;
            }
            if let Some(merged) = try_merge(current, ranges[looking_at]) {
                current = merged;
                looking_at += 1;
            } else {
                break;
            }
        }

        result.push(current);

        idx = looking_at;
    }

    result
}

/// Constraint: ranges must be merged and sorted
fn find_gap(ranges: &[(i32, i32)]) -> Vec<i32> {
    ranges
        .windows(2)
        .flat_map(|window| window[0].1 + 1..window[1].0)
        .filter(|&value| (0..=4000000).contains(&value))
        .collect()
}

fn solve_part1(input: &str, row: i32) -> i32 {
    let ranges: Vec<(i32, i32)> = input
        .lines()
        .map(parse_line)
        .filter_map(|(sensor, beacon)| ranges_at_row(sensor, beacon, row))
        .collect();

    merge_ranges(ranges)
        .into_iter()
        .map(|(from, to)| to - from)
        .sum()
}

impl Solve<i32, u64> for Day15 {
    fn part1(input: &str) -> i32 {
        solve_part1(input, 2_000_000)
    }

    fn part2(input: &str) -> u64 {
        let scanners: Vec<_> = input.lines().map(parse_line).collect();
        for row in 0..4_000_000 {
            let ranges: Vec<(i32, i32)> = scanners
                .iter()
                .cloned()
                .filter_map(|(sensor, beacon)| ranges_at_row(sensor, beacon, row))
                .collect();

            let merged = merge_ranges(ranges);
            let possible_cols = find_gap(&merged);

            if let Some(&col) = possible_cols.first() {
                return col as u64 * 4_000_000u64 + row as u64;
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"#;

    #[test]
    fn example_p1() {
        let result = solve_part1(EXAMPLE, 10);
        let expected = 26;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day15::part2(EXAMPLE);
        let expected = 56000011;

        assert_eq!(result, expected);
    }
}
