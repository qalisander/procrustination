use std::collections::HashSet;
use std::iter;
use std::ops::Div;
use advent_2022_rs::get_input_str;
use itertools::Itertools;
use derive_more::*;

// https://adventofcode.com/2022/day/9

type Ans1 = usize;
type Ans2 = u32;

pub fn rope_bridge_1(input: &str) -> Ans1 {
    let dirs = parse(input).0;

    let mut head = Coord(0, 0);
    let mut tail = Coord(0, 0);
    let mut tails = HashSet::from([tail]);

    for dir in dirs {
        let new_head = head + dir.into();
        if tail.dist(new_head) > 1 {
            tail = head;
        }
        head = new_head;
        tails.insert(tail);
    }

    tails.len()
}

pub fn rope_bridge_2(input: &str) -> Ans2 {
    let parsed = parse(input);

    todo!("2")
}

#[derive(Debug)]
struct Parsed(Vec<Dir>);

#[derive(Debug, Copy, Clone)]
enum Dir{
    R,
    L,
    U,
    D,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Add)]
struct Coord(i32, i32);

impl From<Dir> for Coord {
    fn from(value: Dir) -> Self {
        match value {
            Dir::R => Coord(1, 0),
            Dir::L => Coord(-1, 0),
            Dir::U => Coord(0, -1),
            Dir::D => Coord(0, 1),
        }
    }
}

impl Coord {
    fn dist(&self, rhs: Coord) -> i32{
        (self.0 - rhs.0).abs().max((self.1 - rhs.1).abs())
    }
}

fn parse(str: &str) -> Parsed {
    let vec = str.lines().flat_map(|l| {
        let (dir, steps) = l.split_once(' ').expect("Line splitted");
        let dir = match dir {
            "U" => { Dir::U }
            "D" => { Dir::D }
            "L" => { Dir::L }
            "R" => { Dir::R }
            dir => panic!("Invalid dir '{dir}'")
        };
        let steps: usize = steps.parse().expect("Steps count");
        iter::repeat(dir).take(steps)
    }).collect();
    Parsed(vec)
}

fn main() {
    let str = get_input_str(file!());
    let ans = rope_bridge_1(&str);
    println!("Part 1: {ans}");
    let ans = rope_bridge_2(&str);
    println!("Part 2: {ans}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;

    fn get_input() -> &'static str {
        INPUT.strip_prefix('\n').unwrap().strip_suffix('\n').unwrap()
    }

    #[test]
    fn parse_test() {
        let parsed = parse(get_input());
        dbg!(&parsed);
    }

    #[test]
    fn test_1() {
        let expected = 13;
        let ans = rope_bridge_1(get_input());
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_2() {
        let expected = todo!();
        let ans = rope_bridge_2(get_input());
        assert_eq!(ans, expected);
    }
}
