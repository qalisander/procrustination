use advent_2022_rs::get_input_str;
use derive_more::Add;
use itertools::{iproduct, Itertools};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::ops::Index;

// https://adventofcode.com/2022/day/23
// outside your scan, more empty ground extends a long way in every direction.
// If no other Elves are in one of those eight positions, the Elf does not do anything during this round.
// If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step.
// If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
// If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step.
// If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes moving east one step.
// If two or more Elves propose moving to the same position, none of those Elves move.

type Ans1 = i32;
type Ans2 = i32;

pub fn unstable_diffusion_1(input: &str) -> Ans1 {
    let mut elfs = parse(input);
    let mut dirs = Dir::new();

    for _ in 0..10 {
        let mut new_to_prev: HashMap<Coord, Vec<Coord>> =
            elfs.0.iter().cloned().map(|elf| (elf, vec![])).collect();
        for &elf in &elfs.0 {
            if Dir::get_all().all(|d| !elfs.0.contains(&(d + elf))) {
                continue;
            }

            let new_elf = dirs
                .get_deltas()
                .filter_map(|delta| {
                    for adjacent_elf in delta.get_adjacent().map(|delta| delta + elf) {
                        if elfs.0.contains(&adjacent_elf) {
                            return None;
                        }
                    }
                    Some(delta + elf)
                })
                .next();
            if let Some(new_elf) = new_elf {
                new_to_prev.entry(new_elf).or_default().push(elf);
                new_to_prev.remove(&elf);
            }
        }

        let to_remove = new_to_prev
            .iter()
            .filter(|(k, v)| v.len() > 1)
            .map(|(k, v)| *k)
            .collect_vec();
        for new_elf in to_remove {
            for prev_elf in new_to_prev.remove(&new_elf).unwrap() {
                assert!(new_to_prev.insert(prev_elf, vec![]).is_none());
            }
        }

        elfs.0 = new_to_prev.into_keys().collect();
        dirs.next_round();
    }

    let (min_i, max_i) = elfs
        .0
        .iter()
        .map(|elf| elf.0)
        .minmax()
        .into_option()
        .unwrap();
    let (min_j, max_j) = elfs
        .0
        .iter()
        .map(|elf| elf.1)
        .minmax()
        .into_option()
        .unwrap();
    (max_i - min_i + 1) * (max_j - min_j + 1) - elfs.0.len() as i32
}

pub fn unstable_diffusion_2(input: &str) -> Ans2 {
    let parsed = parse(input);
    todo!("2")
}

struct Dir {
    first_indx: i32,
}

const N: Coord = Coord(-1, 0);
const S: Coord = Coord(1, 0);
const W: Coord = Coord(0, -1);
const E: Coord = Coord(0, 1);

impl Dir {
    const DIRS: [Coord; 4] = [N, S, W, E];

    fn new() -> Self {
        Self { first_indx: 0 }
    }

    fn next_round(&mut self) {
        self.first_indx = (self.first_indx + 1) % 4
    }

    fn get_deltas(&self) -> impl Iterator<Item = Coord> + '_ {
        (0..4)
            .map(|i| ((i + self.first_indx) % 4) as usize)
            .map(|i| Self::DIRS[i])
    }

    fn get_all() -> impl Iterator<Item = Coord> {
        [N, S, W, E, N + W, N + E, S + W, S + E].into_iter()
    }
}

#[derive(Debug)]
struct Elfs(HashSet<Coord>);

impl Display for Elfs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (min_i, max_i) = self
            .0
            .iter()
            .map(|elf| elf.0)
            .minmax()
            .into_option()
            .unwrap();
        let (min_j, max_j) = self
            .0
            .iter()
            .map(|elf| elf.1)
            .minmax()
            .into_option()
            .unwrap();
        for i in min_i..=max_i {
            for j in min_j..=max_j {
                if self.0.contains(&Coord(i, j)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// Coordinates abstraction
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Add)]
struct Coord(i32, i32);

impl Coord {
    fn get_adjacent(&self) -> impl Iterator<Item = Coord> + 'static {
        match *self {
            N => [N, N + W, N + E],
            S => [S, S + W, S + E],
            W => [W, W + N, W + S],
            E => [E, E + N, E + S],
            _ => unreachable!(),
        }
        .into_iter()
    }
}

fn parse(str: &str) -> Elfs {
    let elfs = str
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.char_indices().filter_map(move |(j, ch)| match ch {
                '.' => None,
                '#' => Some(Coord(i as i32, j as i32)),
                ch => panic!("Unexpected char '{ch}"),
            })
        })
        .collect();
    Elfs(elfs)
}

fn main() {
    let str = get_input_str(file!());
    let ans = unstable_diffusion_1(&str);
    println!("Part 1: {ans}");
    let ans = unstable_diffusion_2(&str);
    println!("Part 2: {ans}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_0: &str = r#"
.....
..##.
..#..
.....
..##.
.....
    "#;

    const INPUT_1: &str = r#"
..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............
    "#;

    #[test]
    fn parse_test() {
        let parsed = parse(INPUT_1.trim());
        dbg!(&parsed);
    }

    #[test]
    fn test_1() {
        let expected = 110;
        let ans = unstable_diffusion_1(INPUT_1.trim());
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_2() {
        let expected = todo!();
        let ans = unstable_diffusion_2(INPUT_1.trim());
        assert_eq!(ans, expected);
    }
}
