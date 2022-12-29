
use advent_2022_rs::get_input_str;
use itertools::Itertools;

// https://adventofcode.com/2022/day/7

type Ans1 = u32;
type Ans2 = u32;

//To begin, find all of the directories with a total size of at most 100000,
// then calculate the sum of their total sizes.
pub fn no_space_left_on_device_1(input: &str) -> Ans1 {
    let parsed = parse(input);
    todo!("1")
}

pub fn no_space_left_on_device_2(input: &str) -> Ans2 {
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
    let ans = no_space_left_on_device_1(&str);
    println!("Part 1: {ans}");
    let ans = no_space_left_on_device_2(&str);
    println!("Part 2: {ans}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
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
        let expected = 95437;
        let ans = no_space_left_on_device_1(get_input());
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_2() {
        let expected = todo!();
        let ans = no_space_left_on_device_2(get_input());
        assert_eq!(ans, expected);
    }
}
