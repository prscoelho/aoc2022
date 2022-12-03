use std::collections::HashSet;

use crate::runner::Solve;

pub struct Day03;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!("unexpected char"),
    }
}

fn find_repeating_compartment(rucksack: &[char]) -> char {
    let first_compartment: HashSet<char> =
        rucksack.iter().take(rucksack.len() / 2).cloned().collect();

    *rucksack
        .iter()
        .skip(rucksack.len() / 2)
        .find(|ch| first_compartment.contains(&ch))
        .unwrap()
}

fn occurrence(rucksack: &[char]) -> HashSet<char> {
    rucksack.iter().cloned().collect()
}

fn find_repeating_elves(rucksacks: &[Vec<char>]) -> char {
    *rucksacks
        .iter()
        .map(|sack| occurrence(sack))
        .reduce(|acum, item| acum.intersection(&item).cloned().collect::<HashSet<char>>())
        .unwrap()
        .iter()
        .next()
        .unwrap()
}

impl Solve<u32, u32> for Day03 {
    fn part1(input: &str) -> u32 {
        let rucksacks = parse_input(input);

        rucksacks
            .iter()
            .map(|sack| find_repeating_compartment(sack))
            .map(|repeating| priority(repeating))
            .sum()
    }
    fn part2(input: &str) -> u32 {
        let rucksacks = parse_input(input);

        rucksacks
            .chunks(3)
            .map(|chunk| find_repeating_elves(chunk))
            .map(|repeating| priority(repeating))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn correct_prio() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);

        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }

    #[test]
    fn example_p1() {
        let result = Day03::part1(EXAMPLE);
        let expected = 157;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day03::part2(EXAMPLE);
        let expected = 70;

        assert_eq!(result, expected);
    }
}
