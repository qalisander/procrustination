use std::collections::HashSet;
use std::iter::Cycle;
use std::ops::Deref;
use derive_more::Deref;
use advent_2022_rs::get_input_str;
use itertools::Itertools;
use crate::Event::{BeginAdd, EndAdd, Idle};
use crate::Instr::{Addx, Noop};

// https://adventofcode.com/2022/day/10

type Ans1 = i32;
type Ans2 = i32;

pub fn cathode_ray_tube_1(input: &str) -> Ans1 {
    let parsed = parse(input);
    let control_cycles = HashSet::from([20, 60, 100, 140, 180]);

    let mut cycle = 1;
    let mut x = 1;
    let mut fut_x =
    for instr in parsed.0 {
        match instr {
            // TODO: takes 1 cycle to complete
            Noop => {}
            // TODO: takes 2 cycles to complete
            Addx(value) => {
                x += value
            }
        }
    }
    x
}

enum Event{
    Idle,
    BeginAdd(i32),
    EndAdd(i32),
}

impl From<Parsed> for Vec<Event>{
    fn from(parsed: Parsed) -> Self {
        parsed.0.iter().flat_map(|&instr| match instr {
            Noop => vec![Idle].into_iter(),
            Addx(value) => vec![BeginAdd(value), EndAdd(value)].into_iter()
        }).collect()
    }
}

pub fn cathode_ray_tube_2(input: &str) -> Ans2 {
    let parsed = parse(input);
    todo!("2")
}

#[derive(Debug)]
struct Parsed(Vec<Instr>);

#[derive(Debug, Copy, Clone)]
enum Instr{
    Noop,
    Addx(i32),
}


fn parse(str: &str) -> Parsed {
    let vec = str.lines().map(|l| {
        let split = l.split_whitespace().collect_vec();
        match split.len() {
            1 if split[0] == "noop" => Noop,
            2 if split[0] == "addx" => Addx(split[1].parse().expect("Integer parsed")),
            _ => panic!("Invalid line! '{l}'")
        }
    }).collect();
    Parsed(vec)
}

fn main() {
    let str = get_input_str(file!());
    let ans = cathode_ray_tube_1(&str);
    println!("Part 1: {ans}");
    let ans = cathode_ray_tube_2(&str);
    println!("Part 2: {ans}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
addx 15
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
noop
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
        let expected = todo!();
        let ans = cathode_ray_tube_1(get_input());
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_2() {
        let expected = todo!();
        let ans = cathode_ray_tube_2(get_input());
        assert_eq!(ans, expected);
    }
}
