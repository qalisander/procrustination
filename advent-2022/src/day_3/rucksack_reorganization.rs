extern crate core;

use advent_2022_rs::get_input_str;
use itertools::Itertools;
use std::collections::HashSet;

// https://adventofcode.com/2022/day/3

type Ans = i32;
type Ans1 = Ans;
type Ans2 = Ans;

pub fn rucksack_reorganization_1(input: &str) -> Ans1 {
    let parsed = parse(input);
    parsed
        .into_iter()
        .map(|str| {
            let set0: HashSet<char> = str.0.chars().collect();
            let set1: HashSet<char> = str.1.chars().collect();
            *set0
                .intersection(&set1)
                .into_iter()
                .next()
                .expect("One common char")
        })
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

type Parsed<'a> = Vec<(&'a str, &'a str)>;

fn parse(str: &str) -> Parsed {
    str.lines()
        .map(|l| {
            let count = l.chars().count();
            let split_indx = l.char_indices().nth(count / 2).unwrap().0;
            l.split_at(split_indx)
        })
        .collect_vec()
}

pub fn rucksack_reorganization_2(input: &str) -> Ans2 {
    let parsed = parse2(input);
    parsed
        .into_iter()
        .map(|str| {
            let set0: HashSet<char> = str.0.chars().collect();
            let set1: HashSet<char> = str.1.chars().collect();
            let set2: HashSet<char> = str.2.chars().collect();
            *set0
                .intersection(&set1)
                .cloned()
                .collect::<HashSet<_>>()
                .intersection(&set2)
                .into_iter()
                .next()
                .expect("One common char")
        })
        .map(get_score)
        .sum()
}

type Parsed2<'a> = Vec<(&'a str, &'a str, &'a str)>;

fn parse2(str: &str) -> Parsed2 {
    str.lines()
        .batching(|l| l.take(3).collect_tuple::<(_, _, _)>())
        .collect_vec()
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
    fn parse2_test() {
        let parsed = parse2(INPUT.trim());
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
        let expected = 70;
        let ans = rucksack_reorganization_2(INPUT.trim());
        assert_eq!(ans, expected);
    }
}
