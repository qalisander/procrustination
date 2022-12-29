extern crate core;

use advent_2022_rs::get_input_str;
use anyhow::{anyhow, Error, Result};
use derive_more::{Add, AddAssign, Deref, Display, FromStr};
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::Sum;
use std::ops::Deref;
use std::str::FromStr;

// https://adventofcode.com/2022/day/7

type Ans1 = u32;
type Ans2 = u32;

// To begin, find all of the directories with a total size of at most 100000,
// then calculate the sum of their total sizes.
pub fn no_space_left_on_device_1(input: &str) -> Ans1 {
    let parsed = parse(input);

    const MAX_SIZE: Ans1 = 100_000;
    let mut ans: Ans1 = 0;
    let mut dir_stack = vec![];
    let mut fs_stack: Vec<HashMap<Name, Option<Size>>> = vec![];
    for x in parsed.iter() {
        match x {
            Cmd::Cd(name) => match name.deref() {
                ".." => {
                    let name = dir_stack.pop().expect("Parent file exists");
                    let sum = fs_stack
                        .pop()
                        .expect("Parent file exists")
                        .into_values()
                        .fold(Some(Size(0)), |acc, sz| Some(acc? + sz?));
                    let last = fs_stack.last_mut().expect("fs_stack not empty");
                    let size = last
                        .get_mut(&name)
                        .unwrap_or_else(|| panic!("Obj with '{name}' exist"));

                    match sum {
                        None => {
                            println!("Sum wasn't calculated for dir '{name}")
                        }
                        Some(sum) => {
                            if sum.0 <= MAX_SIZE {
                                ans += sum.0;
                            }
                            size.insert(sum);
                        }
                    }
                }
                "/" => {
                    dir_stack.drain(..);
                    fs_stack.drain(..);
                }
                _ => {
                    dir_stack.push(name.clone());
                }
            },
            Cmd::Ls(ls) => fs_stack.push(HashMap::from_iter(ls.iter().map(|obj| match obj {
                FsObj::File(nm, sz) => (nm.clone(), Some(*sz)),
                FsObj::Dir(nm) => (nm.clone(), None),
            }))),
        }
    }
    ans
}

pub fn no_space_left_on_device_2(input: &str) -> Ans2 {
    let parsed = parse(input);
    todo!("2")
}

#[derive(Debug, Deref)]
struct Parsed(Vec<Cmd>);

impl FromStr for Parsed {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Parsed(
            s.split('$')
                .filter(|s| !s.trim().is_empty())
                .map(str::parse)
                .try_collect()?,
        ))
    }
}

#[derive(Debug)]
enum Cmd {
    Cd(Name),
    Ls(Vec<FsObj>),
}

impl FromStr for Cmd {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.trim().lines();
        let cmd_str = lines.next().ok_or_else(|| anyhow!("Empty cmd string"))?;
        let mut cmd = cmd_str.split_whitespace();
        let cmd: Cmd = match cmd.next() {
            Some("ls") => {
                let ls_out = lines.map(str::parse).try_collect()?;
                Cmd::Ls(ls_out)
            }
            Some("cd") => {
                let name = cmd
                    .next()
                    .ok_or_else(|| anyhow!("Invalid cd '{cmd_str}'"))?;
                Cmd::Cd(name.parse()?)
            }
            _ => return Err(anyhow!("Invalid cmd line '{cmd_str}'")),
        };
        Ok(cmd)
    }
}

#[derive(Debug, Clone)]
enum FsObj {
    File(Name, Size),
    Dir(Name),
}

impl FromStr for FsObj {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (dir_or_size, name): (&str, &str) = s
            .split_whitespace()
            .collect_tuple()
            .ok_or_else(|| anyhow!("Invalid fs object '{s}'"))?;

        match dir_or_size {
            "dir" => Ok(FsObj::Dir(name.parse()?)),
            size => Ok(FsObj::File(name.parse()?, size.parse()?)),
        }
    }
}

#[derive(Debug, FromStr, Deref, Clone, Copy, Add)]
struct Size(u32);

impl From<u32> for Size {
    fn from(num: u32) -> Self {
        Self(num)
    }
}

#[derive(Debug, FromStr, Clone, Eq, PartialEq, Hash, Display)]
struct Name(String);

impl Deref for Name {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn parse(str: &str) -> Parsed {
    str.parse().expect("Parsed value")
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
