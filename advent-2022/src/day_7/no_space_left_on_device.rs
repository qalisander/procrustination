use std::iter;
use advent_2022_rs::get_input_str;
use anyhow::{anyhow, Error, Result};
use itertools::Itertools;
use derive_more::FromStr;

// https://adventofcode.com/2022/day/7

type Ans1 = u32;
type Ans2 = u32;

//To begin, find all of the directories with a total size of at most 100000,
// then calculate the sum of their total sizes.
pub fn no_space_left_on_device_1(input: &str) -> Ans1 {
    let parsed = parse(input).expect("Parsed input");
    todo!("1")
}

pub fn no_space_left_on_device_2(input: &str) -> Ans2 {
    let parsed = parse(input).expect("Parsed input");
    todo!("2")
}

type Parsed = Result<Vec<Cmd>>;

#[derive(Debug)]
enum Cmd {
    Cd(Name),
    Ls(Vec<FsObj>),
}

#[derive(Debug)]
enum FsObj {
    File(Name, Size),
    Dir(Name),
}

type Size = u32;

#[derive(Debug, FromStr)]
struct Name(String);


// TODO: cmd from str
fn parse(str: &str) -> Parsed {
    str.lines()
        .peekable()
        .batching(|lines| {
            let cmd_str = lines.next()?;
            // TODO: create separate method parse_cmd
            let mut cmd = cmd_str.split_whitespace();
            let Some("$") = cmd.next() else {
                return Some(Err(anyhow!("Invalid cmd line '{cmd_str}'")))
            };

            let cmd: Cmd = match cmd.next() {
                Some("ls") => {
                    let vec = lines
                        .peeking_take_while(|line| !line.starts_with("$"))
                        .collect_vec();
                    let result = vec
                        .iter()
                        .map(|line_str| {
                            let (dir_or_size, name): (&str, &str) = line_str
                                .split_whitespace()
                                .collect_tuple()
                                .ok_or_else(|| anyhow!("Invalid fs object '{line_str}'"))?;

                            match dir_or_size {
                                "dir" => Ok(FsObj::Dir(name.parse()?)),
                                size => Ok(FsObj::File(name.parse()?, size.parse()?)),
                            }
                        })
                        .try_collect::<_, Vec<_>, Error>();
                    match result {
                        Ok(output) => Cmd::Ls(output),
                        Err(err) => return Some(Err(err)),
                    }
                }
                Some("cd") => {
                    if let Some(str) = cmd.next() {
                        Cmd::Cd(Name(str.to_string()))
                    } else {
                        return Some(Err(anyhow!("Invalid cd '{cmd_str}'")));
                    }
                }
                _ => return Some(Err(anyhow!("Invalid cmd line '{cmd_str}'"))),
            };
            Some(Ok(cmd))
        })
        .try_collect()
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
        INPUT
            .strip_prefix('\n')
            .unwrap()
            .strip_suffix('\n')
            .unwrap()
    }

    #[test]
    fn parse_test() {
        let parsed = parse(get_input()).expect("Parsed input");
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
