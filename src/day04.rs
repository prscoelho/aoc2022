use crate::runner::Solve;

pub struct Day04;

#[derive(Clone, Copy, Debug)]
struct Range {
    start: i32,
    end: i32,
}

fn parse_range(input: &str) -> Range {
    let mut tokens = input.split('-');
    let start = tokens.next().unwrap().parse::<i32>().unwrap();
    let end = tokens.next().unwrap().parse::<i32>().unwrap();

    Range { start, end }
}

fn parse_line(input: &str) -> (Range, Range) {
    let mut ranges = input.split(',').map(parse_range);
    (ranges.next().unwrap(), ranges.next().unwrap())
}

fn parse_input(input: &str) -> Vec<(Range, Range)> {
    input.lines().map(parse_line).collect()
}

fn fully_contains(start: &Range, end: &Range) -> bool {
    start.start >= end.start && start.end <= end.end
        || end.start >= start.start && end.end <= start.end
}

fn check_overlap(left: &Range, right: &Range) -> bool {
    !(left.end < right.start || left.start > right.end)
}

impl Solve<usize, usize> for Day04 {
    fn part1(input: &str) -> usize {
        let ranges = parse_input(input);

        ranges
            .into_iter()
            .filter(|(left, right)| fully_contains(left, right))
            .count()
    }
    fn part2(input: &str) -> usize {
        let ranges = parse_input(input);

        ranges
            .into_iter()
            .filter(|(left, right)| check_overlap(left, right))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn example_p1() {
        let result = Day04::part1(EXAMPLE);
        let expected = 2;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day04::part2(EXAMPLE);
        let expected = 4;

        assert_eq!(result, expected);
    }
}
