
use advent_2022_rs::get_input_str;
use itertools::Itertools;

// https://adventofcode.com/2022/day/9

type Ans1 = todo!();
type Ans2 = todo!();

pub fn rope_bridge_1(input: &str) -> Ans1 {
    let parsed = parse(input);
    todo!("1")
}

pub fn rope_bridge_2(input: &str) -> Ans2 {
    let parsed = parse(input);
    todo!("2")
}

#[derive(Debug)]
struct Parsed;

fn parse(str: &str) -> Parsed {
    todo!("Parse")
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
