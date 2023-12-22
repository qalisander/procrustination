use advent_2022_rs::get_input_str;
use derive_more::*;
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::iter;
use std::ops::Div;

// https://adventofcode.com/2022/day/9

type Ans1 = usize;
type Ans2 = usize;

pub fn rope_bridge_1(input: &str) -> Ans1 {
    let dirs = parse(input).0;

    let mut knots = Knots([Coord(0, 0); 2]);
    let mut tails = HashSet::from([knots.tail()]);

    for dir in dirs {
        let new_head = knots.head() + dir.into();
        if let Some(dir) = knots.tail().get_next_dir(new_head) {
            *knots.tail_mut() += dir;
        }
        *knots.head_mut() = new_head;
        tails.insert(knots.tail());
    }

    tails.len()
}

pub fn rope_bridge_2(input: &str) -> Ans2 {
    let dirs = parse(input).0;
    let mut knots = Knots([Coord(0, 0); 10]);
    let mut tails = HashSet::from([knots.last().copied().unwrap()]);

    for dir in dirs {
        let mut prev_knot = None;
        for knot in knots.iter_mut() {
            match &mut prev_knot {
                None => {
                    // current knot is head
                    *knot += dir.into();
                    prev_knot = Some(*knot);
                }
                Some(prev_knot) => {
                    if let Some(dir) = knot.get_next_dir(*prev_knot) {
                        *knot += dir;
                    }
                    *prev_knot = *knot;
                }
            }
        }
        tails.insert(knots.tail());
    }
    tails.len()
}

#[derive(Debug, Deref, DerefMut)]
struct Knots<const LN: usize>([Coord; LN]);

impl<const LN: usize> Knots<LN> {
    fn head(&self) -> Coord {
        self.first().copied().expect("Head exists")
    }

    fn head_mut(&mut self) -> &mut Coord {
        self.first_mut().expect("Head exists")
    }

    fn tail(&self) -> Coord {
        self.last().copied().expect("Tail exists")
    }

    fn tail_mut(&mut self) -> &mut Coord {
        self.last_mut().expect("Tail exists")
    }
}

impl<const LN: usize> Display for Knots<LN> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (min_i, max_i) = self.0.iter().map(|x| x.0).minmax().into_option().unwrap();
        let (min_j, max_j) = self.0.iter().map(|x| x.1).minmax().into_option().unwrap();
        for i in min_i..=max_i {
            for j in min_j..=max_j {
                if let Some(index) = self.0.iter().position(|&x| Coord(i, j) == x) {
                    if index == 0 {
                        write!(f, "H")?;
                    } else if index < 10 {
                        write!(f, "{}", index)?;
                    } else {
                        panic!("Invalid index! {index}")
                    }
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Parsed(Vec<Dir>);

#[derive(Debug, Copy, Clone)]
enum Dir {
    R,
    L,
    U,
    D,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Add, Sub, AddAssign)]
struct Coord(i32, i32);

impl From<Dir> for Coord {
    fn from(value: Dir) -> Self {
        match value {
            Dir::R => Coord(0, 1),
            Dir::L => Coord(0, -1),
            Dir::U => Coord(-1, 0),
            Dir::D => Coord(1, 0),
        }
    }
}

impl Coord {
    fn dist(&self, rhs: Coord) -> i32 {
        (self.0 - rhs.0).abs().max((self.1 - rhs.1).abs())
    }

    /// Get next direction based on previous knot
    fn get_next_dir(&self, prev: Coord) -> Option<Coord> {
        let delta = prev - *self;
        if delta.0.abs().max(delta.1.abs()) > 1 {
            Some(Coord(delta.0.signum(), delta.1.signum()))
        } else {
            None
        }
    }
}

fn parse(str: &str) -> Parsed {
    let vec = str
        .lines()
        .flat_map(|l| {
            let (dir, steps) = l.split_once(' ').expect("Line split");
            let dir = match dir {
                "U" => Dir::U,
                "D" => Dir::D,
                "L" => Dir::L,
                "R" => Dir::R,
                dir => panic!("Invalid dir '{dir}'"),
            };
            let steps: usize = steps.parse().expect("Steps count");
            iter::repeat(dir).take(steps)
        })
        .collect();
    Parsed(vec)
}

fn main() {
    let str = get_input_str(file!());
    let ans = rope_bridge_1(&str);
    assert_eq!(6563, ans);
    println!("Part 1: {ans}");
    let ans = rope_bridge_2(&str);
    println!("Part 2: {ans}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
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
        let expected = 13;
        let ans = rope_bridge_1(get_input());
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_2_1() {
        let expected = 1;
        let ans = rope_bridge_2(get_input());
        assert_eq!(ans, expected);
    }

    const INPUT_2: &str = r#"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#;

    // TODO: refactor get input_2
    fn get_input_2() -> &'static str {
        INPUT_2
            .strip_prefix('\n')
            .unwrap()
            .strip_suffix('\n')
            .unwrap()
    }

    #[test]
    fn test_2_2() {
        let expected = 36;
        let ans = rope_bridge_2(get_input_2());
        assert_eq!(ans, expected);
    }
}
