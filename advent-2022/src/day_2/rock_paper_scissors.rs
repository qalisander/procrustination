use advent_2022_rs::get_input_str;
use bitflags::bitflags;
use itertools::Itertools;

// https://adventofcode.com/2022/day/2

type Ans = i32;
type Ans1 = Ans;
type Ans2 = Ans;

const Rock: Ans = 0;
const Paper: Ans = 1;
const Scissors: Ans = 2;

pub fn rock_paper_scissors_1(input: &str) -> Ans1 {
    parse(input).0
        .into_iter()
        .map(|(rival, me)| {
            let me = (me as Ans) - ('X' as Ans);
            let rival = (rival as Ans) - ('A' as Ans);

            calculate_score(me, rival)
        })
        .sum()
}

pub fn rock_paper_scissors_2(input: &str) -> Ans2 {
    const LOOSE: Ans = 0;
    const DRAW: Ans = 1;
    const WIN: Ans = 2;

    parse(input).0
        .into_iter()
        .map(|(rival, me)| {
            let my_outcome = (me as Ans) - ('X' as Ans);
            let rival = (rival as Ans) - ('A' as Ans);
            let me = (rival + my_outcome - 1).rem_euclid(3);

            calculate_score(me, rival)
        })
        .sum()
}

fn calculate_score(me: Ans, rival: Ans) -> Ans {
    me + 1
        + if rival == me {
        3
    } else if (rival + 1) % 3 == me {
        6
    } else {
        0
    }
}

#[derive(Debug)]
struct Parsed(Vec<(char, char)>);

fn parse(str: &str) -> Parsed {
    let vec = str
        .lines()
        .map(|l| {
            let (l, r) = l.split_once(' ').expect("Split by ' ");
            (
                l.parse().expect("Parse left value"),
                r.parse().expect("Parse right value"),
            )
        })
        .collect_vec();
    Parsed(vec)
}

fn main() {
    let str = get_input_str(file!());
    let ans = rock_paper_scissors_1(&str);
    println!("Part 1: {ans}");
    let ans = rock_paper_scissors_2(&str);
    println!("Part 2: {ans}");
}

#[cfg(test)]
mod tests {
    use crate::{parse, rock_paper_scissors_1, rock_paper_scissors_2};

    const INPUT: &str = r#"
A Y
B X
C Z
    "#;

    #[test]
    fn parse_test() {
        let parsed = parse(INPUT.trim());
        dbg!(&parsed);
    }

    #[test]
    fn test_1() {
        let expected = 15;
        let ans = rock_paper_scissors_1(INPUT.trim());
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_2() {
        let expected = 12;
        let ans = rock_paper_scissors_2(INPUT.trim());
        assert_eq!(ans, expected);
    }
}
