use crate::runner::Solve;

pub struct Day05;

fn parse_stack(board: &Vec<Vec<char>>, col: usize) -> Vec<char> {
    let row_start = board.len() - 2;
    (0..=row_start)
        .rev()
        .map(|row| board[row][col])
        .take_while(|&ch| ch != ' ')
        .collect()
}

fn parse_drawing(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<_> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    assert!(lines.len() >= 2); // at least two lines of input, one for the stack numbers and 1+ for
                               // stacks, and this is always true because aoc does not lie

    (0..lines[0].len())
        .skip(1)
        .step_by(4)
        .map(|col| parse_stack(&lines, col))
        .collect()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Action {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse_actions(input: &str) -> Vec<Action> {
    input
        .lines()
        .map(|line| line.trim_start_matches("move "))
        .map(|line| {
            let (amount_str, line) = line.split_once(" from ").unwrap();
            let amount = amount_str.parse().unwrap();

            let (from_str, to_str) = line.split_once(" to ").unwrap();
            let from = from_str.parse().unwrap();
            let to = to_str.parse().unwrap();

            Action { amount, from, to }
        })
        .collect()
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Action>) {
    let (board, actions) = input.split_once("\n\n").unwrap();

    let board = parse_drawing(board);
    let actions = parse_actions(actions);

    (board, actions)
}

fn apply_action9000(board: &mut Vec<Vec<char>>, action: Action) {
    // action indices are 1-indexed, while ours are 0-indexed
    let from = action.from - 1;
    let to = action.to - 1;

    for _ in 0..action.amount {
        let value = board[from].pop().unwrap();
        board[to].push(value);
    }
}

fn apply_action9001(board: &mut Vec<Vec<char>>, action: Action) {
    // action indices are 1-indexed, while ours are 0-indexed
    let from = action.from - 1;
    let to = action.to - 1;

    let mut stack = Vec::new();

    for _ in 0..action.amount {
        let value = board[from].pop().unwrap();
        stack.push(value);
    }

    for value in stack.into_iter().rev() {
        board[to].push(value);
    }
}

impl Solve<String, String> for Day05 {
    fn part1(input: &str) -> String {
        let (mut board, actions) = parse_input(input);

        for action in actions {
            apply_action9000(&mut board, action);
        }

        board.iter().map(|stack| stack.last().unwrap()).collect()
    }
    fn part2(input: &str) -> String {
        let (mut board, actions) = parse_input(input);

        for action in actions {
            apply_action9001(&mut board, action);
        }

        board.iter().map(|stack| stack.last().unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn parses_example() {
        let (board, actions) = parse_input(EXAMPLE);
        let expected_board = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let expected_actions = vec![
            Action {
                amount: 1,
                from: 2,
                to: 1,
            },
            Action {
                amount: 3,
                from: 1,
                to: 3,
            },
            Action {
                amount: 2,
                from: 2,
                to: 1,
            },
            Action {
                amount: 1,
                from: 1,
                to: 2,
            },
        ];

        assert_eq!(board, expected_board);
        assert_eq!(actions, expected_actions);
    }

    #[test]
    fn example_p1() {
        let result = Day05::part1(EXAMPLE);
        let expected = String::from("CMZ");

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day05::part2(EXAMPLE);
        let expected = String::from("MCD");

        assert_eq!(result, expected);
    }
}
