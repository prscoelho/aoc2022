use std::collections::HashMap;

use crate::runner::Solve;

pub struct Day08;

#[derive(Debug)]
struct Grid {
    cols: usize,
    rows: usize,
    map: HashMap<(usize, usize), u8>,
}

fn parse_input(input: &str) -> Grid {
    let mut map = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let value = ch.to_digit(10).unwrap() as u8;
            map.insert((row, col), value);
        }
    }

    let rows = map.keys().map(|(row, _)| row).max().unwrap() + 1;
    let cols = map.keys().map(|(_, col)| col).max().unwrap() + 1;

    Grid { map, cols, rows }
}

// I hate this code please don't look at this
fn count_trees_visible(grid: &Grid, &(current_row, current_col): &(usize, usize)) -> [usize; 4] {
    let tile_value = grid.map.get(&(current_row, current_col)).unwrap();

    let a = (1..current_row)
        .rev()
        .flat_map(|row| grid.map.get(&(row, current_col)))
        .take_while(|&value| value < tile_value)
        .count();

    let b = (current_row..grid.rows - 1)
        .skip(1)
        .flat_map(|row| grid.map.get(&(row, current_col)))
        .take_while(|&value| value < tile_value)
        .count();

    let c = (1..current_col)
        .rev()
        .flat_map(|col| grid.map.get(&(current_row, col)))
        .take_while(|&value| value < tile_value)
        .count();
    let d = (current_col..grid.cols - 1)
        .skip(1)
        .flat_map(|col| grid.map.get(&(current_row, col)))
        .take_while(|&value| value < tile_value)
        .count();

    [a + 1, b + 1, c + 1, d + 1]
}

// I hate this code please don't look at this
fn is_visible(grid: &Grid, &(current_row, current_col): &(usize, usize)) -> bool {
    let tile_value = grid.map.get(&(current_row, current_col)).unwrap();

    (0..current_row)
        .rev()
        .flat_map(|row| grid.map.get(&(row, current_col)))
        .all(|value| value < tile_value)
        || (current_row..grid.rows)
            .skip(1)
            .flat_map(|row| grid.map.get(&(row, current_col)))
            .all(|value| value < tile_value)
        || (0..current_col)
            .rev()
            .flat_map(|col| grid.map.get(&(current_row, col)))
            .all(|value| value < tile_value)
        || (current_col..grid.cols)
            .skip(1)
            .flat_map(|col| grid.map.get(&(current_row, col)))
            .all(|value| value < tile_value)
}

fn is_edge(grid: &Grid, position: &(usize, usize)) -> bool {
    position.0 == 0 || position.1 == 0 || position.0 == grid.rows - 1 || position.1 == grid.cols - 1
}

fn scenic_score(sees: [usize; 4]) -> usize {
    sees.into_iter().product()
}

fn best_scenic_score(grid: Grid) -> usize {
    grid.map
        .keys()
        .filter(|p| !is_edge(&grid, p))
        .map(|p| count_trees_visible(&grid, p))
        .map(scenic_score)
        .max()
        .unwrap()
}

fn count_visible(grid: Grid) -> usize {
    grid.map.keys().filter(|p| is_visible(&grid, p)).count()
}

impl Solve<usize, usize> for Day08 {
    fn part1(input: &str) -> usize {
        let grid = parse_input(input);
        count_visible(grid)
    }
    fn part2(input: &str) -> usize {
        let grid = parse_input(input);
        best_scenic_score(grid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"30373
25512
65332
33549
35390
"#;

    #[test]
    fn example_p1() {
        let result = Day08::part1(EXAMPLE);
        let expected = 21;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day08::part2(EXAMPLE);
        let expected = 8;

        assert_eq!(result, expected);
    }
}
