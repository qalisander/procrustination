extern crate core;

use advent_2022_rs::get_input_str;
use itertools::Itertools;

// https://adventofcode.com/2022/day/3

type Ans = i32;
type Ans1 = Ans;
type Ans2 = Ans;

pub fn rucksack_reorganization_1(input: &str) -> Ans1 {
    let parsed = parse(input);
    parsed
        .into_iter()
        .map(|str| str.chars().duplicates().next().expect("Only one duplicate"))
        .map(get_score)
        .sum()
}

fn get_score(ch: char) -> Ans1 {
    match ch {
        'a'..='z' => ch as Ans1 + 1 - 'a' as Ans1,
        'A'..='Z' => ch as Ans1 + 1 - 'A' as Ans1 + get_score('z'),
        ch => panic!("Invalid char '{ch}'"),
    }
}

pub fn rucksack_reorganization_2(input: &str) -> Ans2 {
    let parsed = parse(input);
    todo!("2")
}

type Parsed<'a> = Vec<&'a str>;

fn parse(str: &str) -> Parsed {
    str.lines().collect_vec()
}

fn main() {
    let str = get_input_str(file!());
    let ans = rucksack_reorganization_1(&str);
    println!("Part 1: {ans}");
    let ans = rucksack_reorganization_2(&str);
    println!("Part 2: {ans}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
    "#;

    #[test]
    fn parse_test() {
        let parsed = parse(INPUT.trim());
        dbg!(&parsed);
    }

    #[test]
    fn test_1() {
        let expected = 157;
        let ans = rucksack_reorganization_1(INPUT.trim());
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_2() {
        let expected = todo!();
        let ans = rucksack_reorganization_2(INPUT.trim());
        assert_eq!(ans, expected);
    }
}
