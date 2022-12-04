use crate::runner::Solve;

pub struct Day00;

impl Solve<i32, i32> for Day00 {
    fn part1(input: &str) -> i32 {
        0
    }
    fn part2(input: &str) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#""#;

    #[test]
    fn parses_example() {
        // todo
    }

    #[test]
    fn example_p1() {
        let result = Day00::part1(EXAMPLE);
        let expected = 0;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day00::part2(EXAMPLE);
        let expected = 0;

        assert_eq!(result, expected);
    }
}
