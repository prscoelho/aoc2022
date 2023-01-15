use std::{
    collections::{BinaryHeap, HashSet},
    ops::Add,
};

use crate::runner::Solve;

pub struct Day19;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct ObsidianCost {
    ore: usize,
    clay: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct GeodeCost {
    ore: usize,
    obsidian: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Blueprint {
    ore_cost: usize,
    clay_cost: usize,
    obsidian_cost: ObsidianCost,
    geode_cost: GeodeCost,
}

fn parse_blueprint(input: &str) -> Blueprint {
    let mut tokens = input.split("costs ").skip(1);

    let (ore_cost, _) = tokens.next().unwrap().split_once(' ').unwrap();
    let ore_cost = ore_cost.parse().unwrap();

    let (clay_cost, _) = tokens.next().unwrap().split_once(' ').unwrap();
    let clay_cost = clay_cost.parse().unwrap();

    let obsidian_cost = {
        let mut obsidian_costs = tokens.next().unwrap().split_ascii_whitespace();
        let ore = obsidian_costs.next().unwrap().parse().unwrap();
        let clay = obsidian_costs.nth(2).unwrap().parse().unwrap();
        ObsidianCost { ore, clay }
    };

    let geode_cost = {
        let mut geode_costs = tokens.next().unwrap().split_ascii_whitespace();
        let ore = geode_costs.next().unwrap().parse().unwrap();
        let obsidian = geode_costs.nth(2).unwrap().parse().unwrap();
        GeodeCost { ore, obsidian }
    };

    Blueprint {
        ore_cost,
        clay_cost,
        obsidian_cost,
        geode_cost,
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Action {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

enum Possibility {
    Yes,
    Wait,
    No,
}

impl Action {
    fn is_possible(&self, state: &State, blueprint: &Blueprint) -> Possibility {
        match self {
            Action::Ore => {
                if state.bag.ore >= blueprint.ore_cost {
                    Possibility::Yes
                } else {
                    Possibility::Wait
                }
            }
            Action::Clay => {
                if state.bag.ore >= blueprint.clay_cost {
                    Possibility::Yes
                } else {
                    Possibility::Wait
                }
            }
            Action::Obsidian => {
                if state.bag.ore >= blueprint.obsidian_cost.ore
                    && state.bag.clay >= blueprint.obsidian_cost.clay
                {
                    Possibility::Yes
                } else if state.factory.clay > 0 {
                    Possibility::Wait
                } else {
                    Possibility::No
                }
            }
            Action::Geode => {
                if state.bag.ore >= blueprint.geode_cost.ore
                    && state.bag.obsidian >= blueprint.geode_cost.obsidian
                {
                    Possibility::Yes
                } else if state.factory.obsidian > 0 {
                    Possibility::Wait
                } else {
                    Possibility::No
                }
            }
        }
    }

    fn should_build(&self, state: &State, blueprint: &Blueprint) -> bool {
        let ore = [
            blueprint.ore_cost,
            blueprint.clay_cost,
            blueprint.obsidian_cost.ore,
            blueprint.geode_cost.ore,
        ]
        .into_iter()
        .max()
        .unwrap();
        let clay = blueprint.obsidian_cost.clay;
        let obsidian = blueprint.geode_cost.obsidian;

        match self {
            Action::Ore => state.factory.ore < ore,
            Action::Clay => state.factory.clay < clay,
            Action::Obsidian => state.factory.obsidian < obsidian,
            Action::Geode => true,
        }
    }

    fn as_items(&self) -> Items {
        match self {
            Action::Ore => Items {
                ore: 1,
                ..Default::default()
            },
            Action::Clay => Items {
                clay: 1,
                ..Default::default()
            },
            Action::Obsidian => Items {
                obsidian: 1,
                ..Default::default()
            },
            Action::Geode => Items {
                geode: 1,
                ..Default::default()
            },
        }
    }

    fn cost(&self, bag: &Items, blueprint: &Blueprint) -> Items {
        let mut result = *bag;
        match self {
            Action::Ore => result.ore -= blueprint.ore_cost,
            Action::Clay => result.ore -= blueprint.clay_cost,
            Action::Obsidian => {
                result.ore -= blueprint.obsidian_cost.ore;
                result.clay -= blueprint.obsidian_cost.clay;
            }
            Action::Geode => {
                result.ore -= blueprint.geode_cost.ore;
                result.obsidian -= blueprint.geode_cost.obsidian;
            }
        };
        result
    }
}

const ACTIONS: [Action; 4] = [Action::Geode, Action::Ore, Action::Clay, Action::Obsidian];

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
struct Items {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Add for Items {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct State {
    turns: usize,
    bag: Items,
    factory: Items,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .turns
            .cmp(&self.turns)
            .then_with(|| self.bag.geode.cmp(&other.bag.geode))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn pass_minute(&self) -> Self {
        let mut result = self.clone();
        result.bag = result.bag + result.factory;
        result.turns += 1;

        result
    }

    fn apply(&self, action: &Action, blueprint: &Blueprint) -> Self {
        let factory = self.factory + action.as_items();
        let bag = action.cost(&self.bag, blueprint) + self.factory;

        State {
            factory,
            bag,
            turns: self.turns + 1,
        }
    }
}

fn maximize_geodes(blueprint: Blueprint, time: usize) -> usize {
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();
    let mut start = State::default();
    start.factory.ore = 1;

    seen.insert((start.bag, start.factory));
    heap.push(start);

    while let Some(current) = heap.pop() {
        if current.turns == time {
            return current.bag.geode;
        }

        for action in ACTIONS.iter() {
            match (
                action.is_possible(&current, &blueprint),
                action.should_build(&current, &blueprint),
            ) {
                (Possibility::Yes, true) => {
                    let next_state = current.apply(action, &blueprint);
                    if seen.insert((next_state.bag, next_state.factory)) {
                        heap.push(next_state);
                        if action == &Action::Geode {
                            // we never want to do anything but build a geode, if it's possible
                            break;
                        }
                    }
                }
                (Possibility::Wait, true) => {
                    let mut next_state = current.clone();
                    while let Possibility::Wait = action.is_possible(&next_state, &blueprint) {
                        next_state = next_state.pass_minute();
                    }
                    next_state = next_state.apply(action, &blueprint);
                    if seen.insert((next_state.bag, next_state.factory)) {
                        heap.push(next_state);
                    }
                }
                _ => { /* do nothing */ }
            }
        }
    }

    panic!("Unexpected unreachable");
}

impl Solve<usize, usize> for Day19 {
    fn part1(input: &str) -> usize {
        input
            .lines()
            .map(parse_blueprint)
            .map(|bp| maximize_geodes(bp, 24))
            .enumerate()
            .map(|(idx, geodes)| (idx + 1) * geodes)
            .sum()
    }
    fn part2(input: &str) -> usize {
        input
            .lines()
            .take(3)
            .map(parse_blueprint)
            .map(|bp| maximize_geodes(bp, 32))
            .product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."#;

    #[test]
    fn parses_example() {
        let example1 = EXAMPLE.lines().next().unwrap();
        let bp = parse_blueprint(example1);
        let expected = Blueprint {
            ore_cost: 4,
            clay_cost: 2,
            obsidian_cost: ObsidianCost { ore: 3, clay: 14 },
            geode_cost: GeodeCost {
                ore: 2,
                obsidian: 7,
            },
        };

        assert_eq!(bp, expected);
    }

    #[test]
    fn example_p1() {
        let result = Day19::part1(EXAMPLE);
        let expected = 33;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day19::part2(EXAMPLE);
        let expected = 62;

        assert_eq!(result, expected);
    }
}
