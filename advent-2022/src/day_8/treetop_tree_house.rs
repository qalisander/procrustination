use advent_2022_rs::get_input_str;
use itertools::Itertools;

// https://adventofcode.com/2022/day/8

type Ans1 = u32;
type Ans2 = u32;

pub fn treetop_tree_house_1(input: &str) -> Ans1 {
    let parsed = parse(input);
    todo!("1")
}

pub fn treetop_tree_house_2(input: &str) -> Ans2 {
    let parsed = parse(input);
    todo!("2")
}

#[derive(Debug)]
struct Parsed(Vec<Vec<char>>);

fn parse(str: &str) -> Parsed {
    let vec_vec = str
        .split_whitespace()
        .map(|str| str.chars().collect())
        .collect();
    Parsed(vec_vec)
}

fn main() {
    let str = get_input_str(file!());
    let ans = treetop_tree_house_1(&str);
    println!("Part 1: {ans}");
    let ans = treetop_tree_house_2(&str);
    println!("Part 2: {ans}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
30373
25512
65332
33549
35390
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
        let expected = 21;
        let ans = treetop_tree_house_1(get_input());
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_2() {
        let expected = todo!();
        let ans = treetop_tree_house_2(get_input());
        assert_eq!(ans, expected);
    }
}
