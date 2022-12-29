use derive_more::Deref;
use advent_2022_rs::get_input_str;
use itertools::Itertools;

// https://adventofcode.com/2022/day/6

type Ans1 = usize;
type Ans2 = usize;

pub fn tuning_trouble_1(input: &str) -> Ans1 {
    get_start_indx(input, 4).expect("End of packet is found")
}

fn get_start_indx(input: &str, window_size: usize) -> Option<usize> {
    let parsed = parse(input);
    for begin in 0..(parsed.len() - window_size) {
        let end = begin + window_size;
        if parsed[begin..end].iter().all_unique() {
            return Some(end);
        }
    }
    None
}

pub fn tuning_trouble_2(input: &str) -> Ans2 {
    get_start_indx(input, 14).expect("End of packet is found")
}

#[derive(Deref, Debug)]
struct Parsed(Vec<char>);

fn parse(str: &str) -> Parsed {
    Parsed(str.chars().collect_vec())
}

fn main() {
    let str = get_input_str(file!());
    let ans = tuning_trouble_1(&str);
    println!("Part 1: {ans}");
    let ans = tuning_trouble_2(&str);
    println!("Part 2: {ans}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
mjqjpqmgbljsphdztnvjfqwrcgsmlb
"#;

    const INPUT_1: &str = r#"
bvwbjplbgvbhsrlpgdmjqwftvncz
"#;

    const INPUT_2: &str = r#"
nppdvjthqldpwncqszvftbrmjlhg
"#;

    fn get_input(input: &'static str) -> &'static str {
        input
            .strip_prefix('\n')
            .unwrap()
            .strip_suffix('\n')
            .unwrap()
    }

    #[test]
    fn parse_test() {
        let parsed = parse(get_input(INPUT));
        dbg!(&parsed);
    }

    #[test]
    fn test_1() {
        let expected = 7;
        let ans = tuning_trouble_1(get_input(INPUT));
        assert_eq!(ans, expected);

        let expected = 5;
        let ans = tuning_trouble_1(get_input(INPUT_1));
        assert_eq!(ans, expected);

        let expected = 6;
        let ans = tuning_trouble_1(get_input(INPUT_2));
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_2() {
        let expected = todo!();
        let ans = tuning_trouble_2(get_input(INPUT));
        assert_eq!(ans, expected);
    }
}
