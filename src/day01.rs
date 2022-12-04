use crate::runner::Solve;

fn read_input(input: &str) -> Vec<Vec<i32>> {
    let mut elves = Vec::new();
    for elf_lines in input.trim().split("\n\n") {
        let mut elf = Vec::new();
        for line in elf_lines.split('\n') {
            let number: i32 = line.parse().unwrap();
            elf.push(number);
        }
        elves.push(elf);
    }
    elves
}

pub struct Day01;

impl Solve<i32, i32> for Day01 {
    fn part1(input: &str) -> i32 {
        let elves = read_input(input);
        elves
            .into_iter()
            .map(|elf| elf.into_iter().sum())
            .max()
            .unwrap()
    }

    fn part2(input: &str) -> i32 {
        let elves = read_input(input);

        let mut calories: Vec<i32> = elves.into_iter().map(|elf| elf.into_iter().sum()).collect();
        calories.sort();

        calories.into_iter().rev().take(3).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    #[test]
    fn parses_example() {
        let elves = read_input(EXAMPLE);
        assert_eq!(elves.len(), 5);
        assert_eq!(elves[0].len(), 3);

        assert_eq!(elves[3][1], 8000);
    }

    #[test]
    fn example_p1() {
        let result = Day01::part1(EXAMPLE);
        let expected = 24000;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day01::part2(EXAMPLE);
        let expected = 45000;

        assert_eq!(result, expected);
    }
}
