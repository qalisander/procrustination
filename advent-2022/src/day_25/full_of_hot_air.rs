use advent_2022::get_input_str;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SnafuNum(Vec<SnafuDgt>);

impl From<&str> for SnafuNum {
    fn from(str: &str) -> Self {
        SnafuNum(str.chars().rev().map(char::into).collect_vec())
    }
}

impl Display for SnafuNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().rev().cloned().map(char::from).join("")
        )
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct SnafuDgt(i8);

impl SnafuDgt {
    const MIN_VAL: i8 = -2;
}

impl From<char> for SnafuDgt {
    fn from(ch: char) -> Self {
        SnafuDgt(match ch {
            '=' => SnafuDgt::MIN_VAL,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            ch => panic!("Invalid char '{ch}"),
        })
    }
}

impl From<SnafuDgt> for char {
    fn from(dgt: SnafuDgt) -> Self {
        match dgt.0 {
            SnafuDgt::MIN_VAL => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            num => panic!("Invalid num '{num}'"),
        }
    }
}

impl Default for SnafuNum {
    fn default() -> Self {
        SnafuNum(vec![SnafuDgt::default()])
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
        let vec = self
            .0
            .iter()
            .map(Some)
            .chain(iter::repeat(None))
            .zip(rhs.0.iter().map(Some).chain(iter::repeat(None)))
            .scan(0, |acc, zipped| {
                let sum = match zipped {
                    (Some(SnafuDgt(l)), Some(SnafuDgt(r))) => l + r + *acc,
                    (None, Some(SnafuDgt(n))) | (Some(SnafuDgt(n)), None) => n + *acc,
                    (None, None) if *acc > 0 => *acc,
                    _ => return None,
                };
                *acc = (sum - SnafuDgt::MIN_VAL).div_euclid(5);
                Some(SnafuDgt(
                    (sum - SnafuDgt::MIN_VAL).rem_euclid(5) + SnafuDgt::MIN_VAL,
                ))
            })
            .collect();
        SnafuNum(vec)
    }
}

type Parsed = Vec<SnafuNum>;

fn parse(str: &str) -> Parsed {
    str.lines().map(SnafuNum::from).collect_vec()
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

    fn add_test(left: &str, right: &str, expected: &str) {
        let left: SnafuNum = left.into();
        let right: SnafuNum = right.into();
        let expected: SnafuNum = expected.into();
        assert_eq!(left + right, expected);
    }

    #[test]
    fn test_1() {
        let expected: SnafuNum = "2=-1=0".into();
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
