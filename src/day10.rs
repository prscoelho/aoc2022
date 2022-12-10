use crate::runner::Solve;

pub struct Day10;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn cycle(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

fn parse_line(input: &str) -> Instruction {
    if input.starts_with("noop") {
        Instruction::Noop
    } else {
        let (_, right) = input.split_once(' ').unwrap();
        Instruction::Addx(right.parse().unwrap())
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(parse_line).collect()
}

fn cycle_values(instructions: Vec<Instruction>) -> Vec<(i32, usize)> {
    let mut register_x = 1;
    let mut current_cycle = 1;
    let mut result = vec![(register_x, current_cycle)];

    for instruction in instructions {
        current_cycle += instruction.cycle();
        if let Instruction::Addx(value) = instruction {
            register_x += value;
        }

        result.push((register_x, current_cycle));
    }

    result
}

fn draw(crt: [bool; 240]) -> String {
    let mut result = String::new();

    for (idx, b) in crt.into_iter().enumerate() {
        // intentionally add a new line before the first character, so that "Part2: {}" renders as
        // Part2: \nCRT
        if idx % 40 == 0 {
            result.push('\n');
        }

        if b {
            result.push('@');
        } else {
            result.push(' ');
        }
    }
    result
}

fn value_at_cycle(history: &[(i32, usize)], cycle: usize) -> (i32, usize) {
    let range = cycle - 1..=cycle;
    history
        .iter()
        .find(|(_, current_cycle)| range.contains(current_cycle))
        .unwrap()
        .clone()
}

impl Solve<i32, String> for Day10 {
    fn part1(input: &str) -> i32 {
        let instructions = parse_input(input);

        let history = cycle_values(instructions);
        let markers = [20, 60, 100, 140, 180, 220];
        let mut result = 0;

        for marker in markers {
            let (register, _) = value_at_cycle(&history, marker);
            result += marker as i32 * register;
        }
        result
    }
    fn part2(input: &str) -> String {
        let instructions = parse_input(input);
        let history = cycle_values(instructions);
        let mut crt = [false; 240];

        for cycle in 1..=240 {
            // this is a mess of 0-indexed vs 1-indexed values!
            let (register, _) = value_at_cycle(&history, cycle);
            let cycle0 = (cycle as i32 - 1) % 40;
            let range = register - 1..=register + 1;
            if range.contains(&cycle0) {
                crt[cycle - 1] = true;
            }
        }
        draw(crt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

    #[test]
    fn parses_example() {
        // todo
    }

    #[test]
    fn example_p1() {
        let result = Day10::part1(EXAMPLE);
        let expected = 13140;


        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day10::part2(EXAMPLE);
        let expected = r#"
@@  @@  @@  @@  @@  @@  @@  @@  @@  @@  
@@@   @@@   @@@   @@@   @@@   @@@   @@@ 
@@@@    @@@@    @@@@    @@@@    @@@@    
@@@@@     @@@@@     @@@@@     @@@@@     
@@@@@@      @@@@@@      @@@@@@      @@@@
@@@@@@@       @@@@@@@       @@@@@@@     "#;

        assert_eq!(result, expected);
    }
}
