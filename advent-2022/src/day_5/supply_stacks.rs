use advent_2022_rs::get_input_str;
use itertools::Itertools;
use regex::Regex;
use std::str::FromStr;

// https://adventofcode.com/2022/day/5

type Ans1 = String;
type Ans2 = String;

pub fn supply_stacks_1(input: &str) -> Ans1 {
    let Parsed{mut crates, commands} = parse(input);
    for cmd in commands {
        cmd
    }
    todo!("1")
}

pub fn supply_stacks_2(input: &str) -> Ans2 {
    let parsed = parse(input);
    todo!("2")
}

#[derive(Debug)]
struct Parsed {
    // zero based columns of crates
    crates: Vec<Vec<Crate>>,
    commands: Vec<Cmd>,
}

#[derive(Debug, Copy, Clone)]
struct Cmd {
    /// Crates ammount to move
    amount: usize,
    /// Zero based index
    from: usize,
    /// Zero based index
    to: usize,
}

type Crate = char;

fn parse(str: &str) -> Parsed {
    let mut lines = str.lines().peekable();
    let raw_crates = lines
        .peeking_take_while(|l| !matches!(l.chars().nth(1), Some(ch) if ch.is_numeric()))
        .map(|l| l.chars().collect_vec()) // TODO: why it doesnt work and..
        .collect_vec();
    let column_count: usize = lines
        .next()
        .expect("Not input end")
        .split_whitespace()
        .flat_map(usize::from_str)
        .max()
        .expect("At least one");
    let mut crates = vec![vec![]; column_count];
    for i in 0..column_count {
        let raw_i = i * 4 + 1;
        crates[i].extend(
            raw_crates
                .iter()
                .rev()
                .flat_map(|raw_chars| raw_chars.get(raw_i)),
        )
    }

    let re = Regex::new(r"move (\d) from (\d) to (\d)").unwrap();
    let commands = lines
        .flat_map(|line| {
            let cap = re.captures(line)?;
            Some(Cmd {
                amount: cap[1].parse().ok()?,
                from: cap[2].parse().ok()?,
                to: cap[3].parse().ok()?,
            })
        })
        .collect_vec();

    Parsed { crates, commands }
}

fn main() {
    let str = get_input_str(file!());
    let ans = supply_stacks_1(&str);
    println!("Part 1: {ans}");
    let ans = supply_stacks_2(&str);
    println!("Part 2: {ans}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

    fn get_input() -> &'static str {
        INPUT
            .strip_prefix('\n')
            .unwrap()
            .strip_suffix('\n')
            .unwrap()
    }

    #[test]
    fn parse_test() {
        let parsed = parse(get_input());
        dbg!(&parsed);
    }

    #[test]
    fn test_1() {
        let expected = "CMZ";
        let ans = supply_stacks_1(get_input());
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_2() {
        let expected = "";
        let ans = supply_stacks_2(get_input());
        assert_eq!(ans, expected);
    }
}
