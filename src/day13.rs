use std::cmp::Ordering;

use crate::runner::Solve;

pub struct Day13;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Value {
    Integer(i32),
    List(Vec<Value>),
}

fn parse_value(input: &str) -> (&str, Value) {
    if let Some(mut input) = input.strip_prefix('[') {
        let mut result = Vec::new();
        while &input[0..1] != "]" {
            if &input[0..1] == "," {
                input = &input[1..];
            }

            let (rest, value) = parse_value(input);
            input = rest;
            result.push(value);
        }

        (&input[1..], Value::List(result))
    } else {
        let n = input[0..2]
            .chars()
            .take_while(|ch| char::is_numeric(*ch))
            .count();

        let number = input[0..n].parse().unwrap();
        (&input[n..], Value::Integer(number))
    }
}

fn parse_input(input: &str) -> Vec<Value> {
    input
        .split("\n\n")
        .flat_map(str::lines)
        .map(parse_value)
        .map(|tuple| tuple.1)
        .collect()
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Integer(v1), Value::Integer(v2)) => v1.cmp(v2),
            (Value::List(l1), Value::List(l2)) => l1.cmp(l2),
            (Value::List(l1), value2) => l1.cmp(&vec![value2.clone()]),
            (value1, Value::List(l2)) => vec![value1.clone()].cmp(l2),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solve<usize, usize> for Day13 {
    fn part1(input: &str) -> usize {
        let values = parse_input(input);

        values
            .chunks_exact(2)
            .enumerate()
            .filter(|(_, pair)| pair[0].cmp(&pair[1]) == Ordering::Less)
            .map(|(idx, _)| idx + 1)
            .sum()
    }

    fn part2(input: &str) -> usize {
        let mut values = parse_input(input);
        let dividers = parse_input(
            r#"[[2]]
[[6]]"#,
        );
        values.extend(dividers.clone());

        values.sort();

        dividers
            .into_iter()
            .filter_map(|divider| values.binary_search(&divider).ok())
            .map(|idx| idx + 1)
            .product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    #[test]
    fn parses_example() {
        let input = "[[4,4],4,4]";

        let expected = Value::List(vec![
            Value::List(vec![Value::Integer(4), Value::Integer(4)]),
            Value::Integer(4),
            Value::Integer(4),
        ]);

        assert_eq!(expected, parse_value(input).1);
    }

    #[test]
    fn example_p1() {
        let result = Day13::part1(EXAMPLE);
        let expected = 13;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day13::part2(EXAMPLE);
        let expected = 140;

        assert_eq!(result, expected);
    }
}
