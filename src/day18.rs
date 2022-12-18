use std::collections::HashSet;

use crate::runner::Solve;

pub struct Day18;

fn parse_line(input: &str) -> Vec<i32> {
    input.split(',').flat_map(str::parse::<i32>).collect()
}

fn parse_input(input: &str) -> HashSet<Vec<i32>> {
    input.lines().map(parse_line).collect()
}

fn neighbours(position: &[i32]) -> Vec<Vec<i32>> {
    (0..3)
        .flat_map(|idx| {
            [-1, 1].iter().map(move |d| {
                let mut neighbour = position.to_owned();
                neighbour[idx] += d;
                neighbour
            })
        })
        .collect()
}

fn count_adjacent(position: &[i32], cubes: &HashSet<Vec<i32>>) -> usize {
    neighbours(position)
        .into_iter()
        .filter(|neighbour| !cubes.contains(neighbour))
        .count()
}

#[derive(Debug)]
struct Bounds {
    x_min: i32,
    x_max: i32,

    y_min: i32,
    y_max: i32,

    z_min: i32,
    z_max: i32,
}

fn within_bounds(position: &[i32], bounds: &Bounds) -> bool {
    position[0] >= bounds.x_min
        && position[0] <= bounds.x_max
        && position[1] >= bounds.y_min
        && position[1] <= bounds.y_max
        && position[2] >= bounds.z_min
        && position[2] <= bounds.z_max
}

fn inner_cubes(cubes: &HashSet<Vec<i32>>) -> HashSet<Vec<i32>> {
    let x_min = cubes.iter().map(|c| c[0]).min().unwrap() - 1;
    let x_max = cubes.iter().map(|c| c[0]).max().unwrap() + 1;

    let y_min = cubes.iter().map(|c| c[1]).min().unwrap() - 1;
    let y_max = cubes.iter().map(|c| c[1]).max().unwrap() + 1;

    let z_min = cubes.iter().map(|c| c[2]).min().unwrap() - 1;
    let z_max = cubes.iter().map(|c| c[2]).max().unwrap() + 1;
    let bounds = Bounds {
        x_min,
        x_max,
        y_min,
        y_max,
        z_min,
        z_max,
    };

    let mut inside_positions = HashSet::new();
    let mut seen = HashSet::new();

    for x in x_min..=x_max {
        for y in y_min..=y_max {
            for z in z_min..=z_max {
                let start = vec![x, y, z];
                if !seen.insert(start.clone()) {
                    continue;
                }
                let mut outside = false;
                let mut island = HashSet::new();
                let mut visit = vec![start.clone()];
                island.insert(start.clone());

                let kind = cubes.contains(&start);

                while let Some(current) = visit.pop() {
                    for neighbour in neighbours(&current) {
                        if kind == cubes.contains(&neighbour) && seen.insert(neighbour.clone()) {
                            if within_bounds(&neighbour, &bounds) {
                                island.insert(neighbour.clone());
                                visit.push(neighbour);
                            } else {
                                outside = true;
                            }
                        }
                    }
                }

                if !outside {
                    inside_positions.extend(island);
                }
            }
        }
    }

    inside_positions
}

impl Solve<usize, usize> for Day18 {
    fn part1(input: &str) -> usize {
        let cubes = parse_input(input);

        cubes.iter().map(|cube| count_adjacent(cube, &cubes)).sum()
    }
    fn part2(input: &str) -> usize {
        let cubes = parse_input(input);
        let inside_positions = inner_cubes(&cubes);

        inside_positions
            .iter()
            .map(|cube| count_adjacent(cube, &inside_positions))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#;

    #[test]
    fn example_p1() {
        let result = Day18::part1(EXAMPLE);
        let expected = 64;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day18::part2(EXAMPLE);
        let expected = 58;

        assert_eq!(result, expected);
    }
}
