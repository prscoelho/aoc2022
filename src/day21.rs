use std::collections::HashMap;

use crate::runner::Solve;

pub struct Day21;

#[derive(Clone, Debug)]
enum Statement {
    Value(i64),
    Operation { left: String, operation: String, right: String }
}

fn parse_line(input: &str) -> (String, Statement) {
    let (lhs, rhs) = input.split_once(": ").unwrap();

    let tokens: Vec<_> = rhs.split_ascii_whitespace().collect();

    let statement = if tokens.len() == 1 {
        Statement::Value(tokens[0].parse().unwrap())
    } else {
        let left = tokens[0].to_owned();
        let operation = tokens[1].to_owned();
        let right = tokens[2].to_owned();
        
        Statement::Operation { left, operation, right }
    };

    (lhs.to_owned(), statement)
}

fn parse_input(input: &str) -> HashMap<String, Statement> {
    input.lines().map(parse_line).collect()
}

fn value(lhs: &str, map: &mut HashMap<String, Statement>) -> i64 {
    // avoid recomputing the same value twice by removing it and inserting it with the computed
    // value worst case scenario the value is removed and inserted right after
    let value = match map.remove(lhs) {
        Some(Statement::Value(value)) => { value },
        Some(Statement::Operation { left, operation, right }) => {
            let left_value = value(&left, map);
            let right_value = value(&right, map);

            let value = match operation.as_str() {
                "+" => left_value + right_value,
                "-" => left_value - right_value,
                "*" => left_value * right_value,
                "/" => left_value / right_value,
                _ => panic!("Unexpected op")
            };

            value
        }
        _ => panic!("Unexpected couldn't find lhs in map")
    };

    map.insert(lhs.to_owned(), Statement::Value(value));
    value
}

fn compare(input_value: i64, original: &HashMap<String, Statement>) -> i64 {
    let mut map = original.clone();
    let root = map.remove("root").unwrap();
    if let Statement::Operation { left, operation: _, right } = root {
        map.insert(String::from("humn"), Statement::Value(input_value));
        let left_value = value(&left, &mut map);
        let right_value = value(&right, &mut map);

        left_value - right_value
    } else {
        panic!("Root is not an operation value");
    }
}

/// Bisect a root value.
///
/// Left and right must have different signs, panics otherwise.
fn bisection_search(map: &HashMap<String, Statement>, mut left: i64, mut right: i64) -> Option<i64> {
    let left_sign = compare(left, map).signum();
    let right_sign = compare(right, map).signum();
    assert_ne!(left_sign, right_sign);

    while left < right {
        let right_value = compare(right, map);
    
        let mid = (left + right) / 2;
        let mid_value = compare(mid, map);
        if mid_value == 0 {
            return Some(mid);
        }

        if right_value.signum() == mid_value.signum() {
            right = mid;
        } else {
            left = mid;
        }
    }

    None
}

fn bisect(map: &HashMap<String, Statement>) -> i64 {
    // not sure the right way to calculate the step value.
    // we need the lowest number that achieves this zero, I think. It's not specified.
    // I found more values that result in zero.
    let intervals: Vec<_> = (0..i64::MAX).step_by(i64::MAX as usize / 10000000).collect();

    for interval in intervals.windows(2) {
        let left = interval[0];
        let left_value = compare(left, &map);
        let right = interval[1];
        let right_value = compare(right, &map);

        // A zero may exist in this interval if the signs don't match
        // Because we're dealing with integer values, the zero may not exist
        // Due to a jump from -1 to 1, for example.
        if left_value.signum() != right_value.signum() {
            if let Some(idx) = bisection_search(map, left, right) {
                return idx;
            }
        }
    }
    panic!("Couldnt find the zero");

}
impl Solve<i64, i64> for Day21 {
    fn part1(input: &str) -> i64 {
        let mut map = parse_input(input);
        value("root", &mut map)
    }
    fn part2(input: &str) -> i64 {
        let map = parse_input(input);

        let value = bisect(&map);
        assert_eq!(compare(value, &map), 0);

        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#;

    #[test]
    fn example_p1() {
        let result = Day21::part1(EXAMPLE);
        let expected = 152;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day21::part2(EXAMPLE);
        let expected = 301;

        assert_eq!(result, expected);
    }
}
