use crate::runner::Solve;

pub struct Day20;

fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect()
}

fn build_ring(list: &[i64]) -> Vec<(usize, usize)> {
    let mut result = vec![(0, 0); list.len()];

    for idx in 1..list.len() {
        result[idx - 1].1 = idx;
        result[idx].0 = idx - 1;
    }

    result[0].0 = list.len() - 1;
    result[list.len() - 1].1 = 0;

    result
}

// Better name?
fn find(ring: &[(usize, usize)], mut idx: usize, mut times: i64) -> usize {
    while times != 0 {
        if times > 0 {
            idx = ring[idx].1;
            times -= 1;
        } else {
            idx = ring[idx].0;
            times += 1;
        }
    }
    idx
}

fn mix(list: &[i64], ring: &mut [(usize, usize)]) {
    for idx in 0..ring.len() {
        let times = list[idx] % (list.len() - 1) as i64;
        if times == 0 {
            continue;
        }

        let goal = find(&ring, idx, times);

        // connect current.prev to current.next
        let prev = ring[idx].0;
        let next = ring[idx].1;
        ring[prev].1 = next;
        ring[next].0 = prev;

        if times > 0 {
            // move between goal and next
            let next = ring[goal].1;

            ring[next].0 = idx;
            ring[goal].1 = idx;

            ring[idx].0 = goal;
            ring[idx].1 = next;
        } else {
            // move between prev and goal
            let prev = ring[goal].0;

            ring[prev].1 = idx;
            ring[goal].0 = idx;

            ring[idx].0 = prev;
            ring[idx].1 = goal;
        }
    }
}

impl Solve<i64, i64> for Day20 {
    fn part1(input: &str) -> i64 {
        let values = parse_input(input);
        let mut ring = build_ring(&values);

        mix(&values, &mut ring);

        // find the zero
        let idx = values.iter().position(|&v| v == 0).unwrap();

        [1000, 2000, 3000]
            .into_iter()
            .map(|after| find(&ring, idx, after))
            .map(|idx| values[idx])
            .sum()
    }
    fn part2(input: &str) -> i64 {
        let multiply_by = 811_589_153;
        let values: Vec<_> = parse_input(input)
            .into_iter()
            .map(|v| v * multiply_by)
            .collect();
        let mut ring = build_ring(&values);

        for _ in 0..10 {
            mix(&values, &mut ring);
        }

        // find the zero
        let idx = values.iter().position(|&v| v == 0).unwrap();

        [1000, 2000, 3000]
            .into_iter()
            .map(|after| find(&ring, idx, after % values.len() as i64))
            .map(|idx| values[idx])
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"1
2
-3
3
-2
0
4"#;

    #[test]
    fn parses_example() {
        let expected = vec![1, 2, -3, 3, -2, 0, 4];
        assert_eq!(parse_input(EXAMPLE), expected);
    }

    #[test]
    fn example_p1() {
        let result = Day20::part1(EXAMPLE);
        let expected = 3;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day20::part2(EXAMPLE);
        let expected = 1623178306;

        assert_eq!(result, expected);
    }
}
