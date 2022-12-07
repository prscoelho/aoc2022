use std::collections::HashSet;

use crate::runner::Solve;

pub struct Day06;

fn length_til_nth_unique(word: &str, n: usize) -> usize {
    let chars: Vec<char> = word.chars().collect();
    for (idx, window) in chars.windows(n).enumerate() {
        if window.iter().cloned().collect::<HashSet<char>>().len() == n {
            return idx + n;
        }
    }
    usize::MAX
}

impl Solve<usize, usize> for Day06 {
    fn part1(input: &str) -> usize {
        length_til_nth_unique(input, 4)
    }
    fn part2(input: &str) -> usize {
        length_til_nth_unique(input, 14)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE0: &str = r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#;

    #[test]
    fn example_p1() {
        let result = Day06::part1(EXAMPLE0);
        let expected = 7;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day06::part2(EXAMPLE0);
        let expected = 19;

        assert_eq!(result, expected);
    }
}
