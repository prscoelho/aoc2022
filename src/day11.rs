use crate::runner::Solve;

pub struct Day11;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Operation {
    MultiplySelf,
    Plus(u64),
    Multiply(u64),
}

fn parse_operation(input: &str) -> Operation {
    let input = input.trim_start_matches("  Operation: new = ");

    if input == "old * old" {
        Operation::MultiplySelf
    } else {
        let mut tokens = input.split_ascii_whitespace().skip(1);
        let op = tokens.next().unwrap();
        let value: u64 = tokens.next().unwrap().parse().unwrap();

        match op {
            "*" => Operation::Multiply(value),
            "+" => Operation::Plus(value),
            _ => panic!("expected a * or +"),
        }
    }
}

fn parse_items(input: &str) -> Vec<u64> {
    let input = input.trim_start_matches("  Starting items: ");
    input.split(", ").map(|s| s.parse().unwrap()).collect()
}

fn parse_num_at_end(input: &str) -> u64 {
    input
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap()
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisible_by: u64,
    throw_true: usize,
    throw_false: usize,
}

fn parse_monkey(input: &str) -> Monkey {
    let lines: Vec<&str> = input.lines().collect();
    let items = parse_items(lines[1]);
    let operation = parse_operation(lines[2]);
    let divisible_by = parse_num_at_end(lines[3]);
    let throw_true = parse_num_at_end(lines[4]) as usize;
    let throw_false = parse_num_at_end(lines[5]) as usize;

    Monkey {
        items,
        operation,
        divisible_by,
        throw_true,
        throw_false,
    }
}

fn calculate_worry(item: u64, operation: Operation) -> u64 {
    match operation {
        Operation::MultiplySelf => item * item,
        Operation::Plus(v) => item + v,
        Operation::Multiply(v) => item * v,
    }
}

/// Constraint: monkeys.len() == inspections.len()
fn round(monkeys: &mut [Monkey], inspections: &mut [usize], worry_division: u64) {
    assert!(monkeys.len() == inspections.len());

    // because divisible_by is always a prime number, we can mod worry by the product of these values
    // without affecting the conditions in any way, but allowing us to keep worry between 0..max
    let max: u64 = monkeys.iter().map(|monkey| monkey.divisible_by).product();

    for monkey_idx in 0..monkeys.len() {
        let items: Vec<u64> = monkeys[monkey_idx].items.drain(..).collect();
        for item in items {
            inspections[monkey_idx] += 1;
            let worry = calculate_worry(item, monkeys[monkey_idx].operation) / worry_division;

            let throw_to_idx = if worry % monkeys[monkey_idx].divisible_by == 0 {
                monkeys[monkey_idx].throw_true
            } else {
                monkeys[monkey_idx].throw_false
            };

            monkeys[throw_to_idx].items.push(worry % max);
        }
    }
}
fn monkey_business(mut inspections: Vec<usize>) -> usize {
    inspections.sort();

    inspections.into_iter().rev().take(2).product()
}

impl Solve<usize, usize> for Day11 {
    fn part1(input: &str) -> usize {
        let mut monkeys: Vec<Monkey> = input.split("\n\n").map(parse_monkey).collect();
        let mut inspections = vec![0; monkeys.len()];

        for _ in 0..20 {
            round(&mut monkeys, &mut inspections, 3);
        }

        monkey_business(inspections)
    }
    fn part2(input: &str) -> usize {
        let mut monkeys: Vec<Monkey> = input.split("\n\n").map(parse_monkey).collect();
        let mut inspections = vec![0; monkeys.len()];

        for _ in 0..10_000 {
            round(&mut monkeys, &mut inspections, 1);
        }
        monkey_business(inspections)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;

    #[test]
    fn parses_operation() {
        assert_eq!(
            parse_operation("  Operation: new = old + 19"),
            Operation::Plus(19)
        );
    }

    #[test]
    fn parses_operation_self() {
        assert_eq!(
            parse_operation("  Operation: new = old * old"),
            Operation::MultiplySelf
        );
    }

    #[test]
    fn example_p1() {
        let result = Day11::part1(EXAMPLE);
        let expected = 10605;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day11::part2(EXAMPLE);
        let expected = 2713310158;

        assert_eq!(result, expected);
    }
}
