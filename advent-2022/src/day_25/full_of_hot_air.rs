use advent_2022_rs::get_input_str;
use derive_more::{Display, Sum};
use itertools::Itertools;
use std::iter;
use std::iter::Sum;
use std::ops::Add;

// https://adventofcode.com/2022/day/25

type Ans1 = SnafuNum;
type Ans2 = SnafuNum;

pub fn full_of_hot_air_1(input: &str) -> Ans1 {
    let parsed = parse(input);
    parsed.into_iter().sum()
}

pub fn full_of_hot_air_2(input: &str) -> Ans2 {
    let parsed = parse(input);
    todo!("2")
}

// TODO: may be better have array of snafuDigit inside
#[derive(Debug, Clone, Eq, PartialEq, Display)]
pub struct SnafuNum(String);

impl Default for SnafuNum {
    fn default() -> Self {
        SnafuNum("0".to_string())
    }
}

impl Sum for SnafuNum {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(SnafuNum::default(), |acc, num| acc + num)
    }
}

impl Add for SnafuNum {
    type Output = SnafuNum;

    fn add(self, rhs: Self) -> Self::Output {
        let str: String = self
            .0
            .chars()
            .rev().map(Some)
            .chain(iter::repeat(None))
            .zip(rhs.0.chars().rev().map(Some).chain(iter::repeat(None)))
            .scan(0, |acc, zipped| {
                let sum = match zipped {
                    (Some(l), Some(r)) => {
                        let (l, r) = (map_ch(l), map_ch(r));
                        l + r + *acc
                    }
                    (None, Some(n)) | (Some(n), None) => {
                        let n = map_ch(n);
                        n + *acc
                    }
                    (None, None) if *acc > 0 => {
                        *acc
                    }
                    _ => return None
                };
                *acc = (sum - MIN_VAL).div_euclid(5); // (8 - (-2) (= 10)) / 5 (=2)
                Some(map_num((sum - MIN_VAL).rem_euclid(5) + MIN_VAL)) // (8 - (-2) % 5 (=0)) -2 (=-2)
            })
            .collect();
        SnafuNum(str.chars().rev().collect())
    }
}

const MIN_VAL: i32 = -2;
fn map_ch(ch: char) -> i32 {
    match ch {
        '=' => MIN_VAL,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        ch => panic!("Invalid char '{ch}"),
    }
}

fn map_num(num: i32) -> char {
    match num {
        MIN_VAL => '=',
        -1 => '-',
        0 => '0',
        1 => '1',
        2 => '2',
        num => panic!("Invalid num '{num}'"),
    }
}

type Parsed = Vec<SnafuNum>;

fn parse(str: &str) -> Parsed {
    str.lines()
        .map(|line| SnafuNum(line.to_string()))
        .collect_vec()
}

fn main() {
    let str = get_input_str(file!());
    let ans = full_of_hot_air_1(&str);
    println!("Part 1: {ans}");
    let ans = full_of_hot_air_2(&str);
    println!("Part 2: {ans}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
    "#;

    #[test]
    fn parse_test() {
        let parsed = parse(INPUT.trim());
        dbg!(&parsed);
    }

    #[test]
    fn add_test_1() {
        add_test("2-", "11", "1=0");
        add_test("1=", "1=", "11");
    }

    fn add_test(left: &str, right: &str, expected: &str){
        let left = SnafuNum(left.to_string()); //9
        let right = SnafuNum(right.to_string()); //6
        let expected = SnafuNum(expected.to_string()); //15
        assert_eq!(left + right, expected);
    }

    #[test]
    fn test_1() {
        let expected = SnafuNum("2=-1=0".to_string());
        let ans = full_of_hot_air_1(INPUT.trim());
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_2() {
        let expected = todo!();
        let ans = full_of_hot_air_2(INPUT.trim());
        assert_eq!(ans, expected);
    }
}
