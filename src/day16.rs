use std::collections::{hash_map::Entry, BTreeSet, BinaryHeap, HashMap, HashSet};

use crate::runner::Solve;

pub struct Day16;

#[derive(Debug)]
struct Valve {
    name: String,
    connections: HashMap<String, usize>,
    flow_rate: usize,
}

fn parse_valve(input: &str) -> Valve {
    let _ = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";

    let (valve_text, rest) = input.split_once(" has flow rate=").unwrap();
    let name = valve_text.trim_start_matches("Valve ").to_string();

    let (number_str, rest) = rest.split_once("; ").unwrap();
    let flow_rate: usize = number_str.parse().unwrap();

    let connections = rest
        .split_ascii_whitespace()
        .skip(4)
        .map(|s| s.trim_end_matches(','))
        .map(str::to_owned)
        .map(|s| (s, 1))
        .collect();

    Valve {
        name,
        connections,
        flow_rate,
    }
}

fn reachable_from(
    map: &HashMap<String, Valve>,
    position: &str,
    visited: &mut HashSet<String>,
) -> HashMap<String, usize> {
    let node = map.get(position).unwrap();

    let mut result = HashMap::new();

    for (connection, &cost) in node.connections.iter() {
        let neighbour = map.get(connection).unwrap();
        if visited.insert(connection.to_owned()) {
            if neighbour.flow_rate == 0 {
                result.extend(
                    reachable_from(map, connection, visited)
                        .into_iter()
                        .map(|(k, v)| (k, v + cost)),
                );
            } else {
                result.insert(connection.clone(), cost);
            }
        }
    }

    result
}

fn prune_zero_flow(map: HashMap<String, Valve>) -> HashMap<String, Valve> {
    let mut result = HashMap::new();

    for name in map.keys() {
        let node = map.get(name).unwrap();
        if name == "AA" || node.flow_rate > 0 {
            let mut visited = HashSet::new();
            visited.insert(name.to_owned());
            let connections = reachable_from(&map, name, &mut visited);
            let valve = Valve {
                connections,
                name: name.to_owned(),
                flow_rate: node.flow_rate,
            };

            result.insert(name.to_owned(), valve);
        }
    }
    result
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    pressure: usize,
    units: Vec<String>,
    minutes: Vec<usize>,
    open: BTreeSet<String>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .minutes
            .cmp(&self.minutes)
            .then_with(|| self.pressure.cmp(&other.pressure))
            .then_with(|| other.open.len().cmp(&self.open.len()))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct PressureKey {
    open: BTreeSet<String>,
    units: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct PressureValue {
    pressure: usize,
    minutes: Vec<usize>,
}

impl Ord for PressureValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.pressure
            .cmp(&other.pressure)
            .then_with(|| other.minutes.cmp(&self.minutes))
    }
}

impl PartialOrd for PressureValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn should_relax(state: &State, pressures: &mut HashMap<PressureKey, PressureValue>) -> bool {
    let pressure_key = PressureKey {
        open: state.open.clone(),
        units: state.units.clone(),
    };

    let pressure_value = PressureValue {
        pressure: state.pressure,
        minutes: state.minutes.clone(),
    };

    let best_value = pressures
        .entry(pressure_key)
        .or_insert_with(|| pressure_value.clone());

    *best_value > pressure_value
}

fn should_proceed(state: &State, pressures: &mut HashMap<PressureKey, PressureValue>) -> bool {
    let pressure_key = PressureKey {
        open: state.open.clone(),
        units: state.units.clone(),
    };

    let pressure_value = PressureValue {
        pressure: state.pressure,
        minutes: state.minutes.clone(),
    };

    match pressures.entry(pressure_key) {
        Entry::Occupied(mut o) => {
            let best_pressure = o.get_mut();
            if *best_pressure >= pressure_value {
                false
            } else {
                *best_pressure = pressure_value;
                true
            }
        }
        Entry::Vacant(v) => {
            v.insert(pressure_value);
            true
        }
    }
}

fn dfs(map: &HashMap<String, Valve>, units: Vec<String>, minute_start: usize) -> usize {
    let start = State {
        pressure: 0,
        minutes: vec![minute_start; units.len()],
        units,
        open: BTreeSet::new(),
    };
    let mut pressures: HashMap<PressureKey, PressureValue> = HashMap::new();
    pressures.insert(
        PressureKey {
            open: start.open.clone(),
            units: start.units.clone(),
        },
        PressureValue {
            pressure: 0,
            minutes: start.minutes.clone(),
        },
    );
    let mut queue = BinaryHeap::new();
    queue.push(start);

    let mut best = 0;

    while let Some(current) = queue.pop() {
        if current.minutes.iter().any(|&minute| minute >= 30) {
            continue;
        }

        if current.pressure > best {
            best = current.pressure;
        }

        let (unit_idx, _cost) = current
            .minutes
            .iter()
            .enumerate()
            .min_by_key(|(_, minute)| *minute)
            .unwrap();

        let current_valve = map.get(&current.units[unit_idx]).unwrap();

        if should_relax(&current, &mut pressures) {
            continue;
        }

        for (neighbour, cost) in current_valve.connections.iter() {
            for activating in [true, false] {
                if activating && current.open.contains(neighbour) {
                    continue;
                }

                let mut next_open = current.open.clone();

                if activating {
                    next_open.insert(neighbour.to_owned());
                }

                let mut next_minutes = current.minutes.clone();
                next_minutes[unit_idx] += cost + usize::from(activating);

                if next_minutes[unit_idx] >= 30 {
                    continue;
                }

                let next_valve = map.get(neighbour).unwrap();

                let next_pressure = current.pressure
                    + usize::from(activating)
                        * (30 - next_minutes[unit_idx])
                        * next_valve.flow_rate;

                let mut next_units = current.units.clone();
                next_units[unit_idx] = neighbour.clone();

                let next_state = State {
                    pressure: next_pressure,
                    units: next_units,
                    minutes: next_minutes,
                    open: next_open,
                };

                if should_proceed(&next_state, &mut pressures) {
                    queue.push(next_state);
                }
            }
        }
    }

    best
}

fn parse_input(input: &str) -> HashMap<String, Valve> {
    input
        .lines()
        .map(parse_valve)
        .map(|v| (v.name.clone(), v))
        .collect()
}

impl Solve<usize, usize> for Day16 {
    fn part1(input: &str) -> usize {
        let map = parse_input(input);
        let map = prune_zero_flow(map);
        dfs(&map, vec![String::from("AA")], 0)
    }
    fn part2(input: &str) -> usize {
        let map = parse_input(input);
        let map = prune_zero_flow(map);
        dfs(&map, vec![String::from("AA"), String::from("AA")], 4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;

    #[test]
    fn parses_example() {
        let valves = parse_input(EXAMPLE);

        assert_eq!(valves.len(), 10);
        assert_eq!(valves["AA"].flow_rate, 0);
        assert_eq!(
            valves["II"]
                .connections
                .keys()
                .cloned()
                .collect::<HashSet<String>>(),
            vec![String::from("AA"), String::from("JJ")]
                .into_iter()
                .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn example_p1() {
        let result = Day16::part1(EXAMPLE);
        let expected = 1651;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day16::part2(EXAMPLE);
        let expected = 1707;

        assert_eq!(result, expected);
    }
}
