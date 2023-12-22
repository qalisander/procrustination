use advent_2022_rs::get_input_str;
use itertools::Itertools;
use std::any::Any;

// https://adventofcode.com/2022/day/1

type Ans = u32;
type Ans1 = Ans;
type Ans2 = Ans;

pub fn calorie_counting_1(input: &str) -> Ans1 {
    let parsed = parse(input).0;
    parsed
        .iter()
        .map(|elf| elf.iter().sum())
        .max()
        .expect("Input is not empty")
}

pub fn calorie_counting_2(input: &str) -> Ans2 {
    let parsed = parse(input).0;
    parsed
        .iter()
        .map(|elf| elf.iter().sum::<Ans>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}

#[derive(Debug)]
struct Parsed(Vec<Vec<Ans>>);

fn parse(str: &str) -> Parsed {
    let vec_vec = str
        .lines()
        .peekable()
        .batching(|iter| {
            let vec = iter.map_while(|l| l.parse::<Ans>().ok()).collect_vec();
            if vec.is_empty() && iter.peek().is_none() {
                None
            } else {
                Some(vec)
            }
        })
        .filter(|vec| !vec.is_empty())
        .collect_vec();
    Parsed(vec_vec)
}

fn main() {
    let str = get_input_str(file!());
    let ans = calorie_counting_1(&str);
    println!("Part 1: {ans}");
    let ans = calorie_counting_2(&str);
    println!("Part 2: {ans}");
}

#[cfg(test)]
mod tests {
    use crate::{calorie_counting_1, calorie_counting_2, parse};

    const INPUT: &str = r#"
1000
2000
3000


4000

5000
6000

7000
8000
9000

10000
    "#;

    #[test]
    fn parse_test() {
        let parsed = parse(INPUT.trim());
        dbg!(&parsed);
    }

    #[test]
    fn test_1() {
        let expected = 24000;
        let ans = calorie_counting_1(INPUT.trim());
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_2() {
        let expected = 45000;
        let ans = calorie_counting_2(INPUT.trim());
        assert_eq!(ans, expected);
    }
}
