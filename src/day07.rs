use std::{collections::HashMap, ops::Range, str::Lines};

use crate::runner::Solve;

pub struct Day07;

#[derive(Debug)]
enum File {
    File(usize),
    Directory(String),
}

fn process_ls(lines: Lines) -> Vec<File> {
    lines
        .map(|line| {
            let (kind, name) = line.split_once(' ').expect("line should have two words");
            if kind == "dir" {
                File::Directory(name.to_owned())
            } else {
                let size: usize = kind.parse().unwrap();
                File::File(size)
            }
        })
        .collect()
}

// Vec<String> as the directory path
type Filesystem = HashMap<Vec<String>, Vec<File>>;

fn parse_input(input: &str) -> Filesystem {
    let mut filesystem = Filesystem::new();
    let mut wd: Vec<String> = vec![];

    for command in input.strip_prefix("$ ").unwrap().split("$ ") {
        let mut lines = command.lines();
        let first = lines.next().unwrap();
        match first {
            "ls" => {
                let contents = process_ls(lines);
                filesystem.insert(wd.clone(), contents);
            }
            "cd .." => {
                wd.pop();
            }
            "cd /" => {
                wd.clear();
                wd.push(String::from("/"));
            }
            // cd {name}
            _ => {
                let (_, name) = first.split_once(' ').expect("expected `cd {name}`");
                wd.push(name.to_owned());
            }
        }
    }
    filesystem
}


/// Walks through directory and returns the total directory size
/// Also mutates counter with sizes that fit within the supplied range
fn walk_directory(
    filesystem: &Filesystem,
    wd: &mut Vec<String>,
    counter: &mut Vec<usize>,
    range: &Range<usize>,
) -> usize {
    let contents = filesystem.get(wd).unwrap();

    let mut total_size = 0;
    let mut pwd = wd.to_owned();
    for kind in contents.iter() {
        let size = match kind {
            File::File(size) => *size,
            File::Directory(name) => {
                pwd.push(name.to_owned());
                let size = walk_directory(filesystem, &mut pwd, counter, range);
                pwd.pop();
                size
            }
        };

        total_size += size;
    }

    if range.contains(&total_size) {
        counter.push(total_size);
    }

    total_size
}

const PART_1_SIZE_LIMIT: usize = 100_000;
const TOTAL_DISK_SPACE: usize = 70_000_000;
const MAXIMUM_USED_DISK_SPACE: usize = 40_000_000;

impl Solve<usize, usize> for Day07 {
    fn part1(input: &str) -> usize {
        let filesystem = parse_input(input);

        let mut counter = Vec::new();
        let range = 0..PART_1_SIZE_LIMIT + 1;
        let mut wd = vec![String::from("/")];
        walk_directory(&filesystem, &mut wd, &mut counter, &range);
        counter.into_iter().sum()
    }
    fn part2(input: &str) -> usize {
        let filesystem = parse_input(input);

        let mut counter = Vec::new();
        let mut wd = vec![String::from("/")];
        // this is hacky, for sure, but we can reuse count_in_range by using a range that will
        // never match and get the total used_space
        let used_space = walk_directory(&filesystem, &mut wd, &mut counter, &(0..0));

        let minimum_space_required = used_space.saturating_sub(MAXIMUM_USED_DISK_SPACE);
        let range = minimum_space_required..TOTAL_DISK_SPACE;
        walk_directory(&filesystem, &mut wd, &mut counter, &range);
        counter.into_iter().min().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    const INPUT: &str = include_str!("../input/07.input");

    #[test]
    fn example_p1() {
        let result = Day07::part1(EXAMPLE);
        let expected = 95437;

        assert_eq!(result, expected);
    }

    #[test]
    fn example_p2() {
        let result = Day07::part2(EXAMPLE);
        let expected = 24933642;

        assert_eq!(result, expected);
    }

    #[test]
    fn input_p1() {
        assert_eq!(Day07::part1(INPUT), 1141028);
    }
    #[test]
    fn input_p2() {
        assert_eq!(Day07::part2(INPUT), 8278005);
    }
}
