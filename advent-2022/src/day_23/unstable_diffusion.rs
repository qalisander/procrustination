use advent_2022_rs::get_input_str;
use itertools::Itertools;

// https://adventofcode.com/2022/day/23

type Ans1 = i32;
type Ans2 = i32;

pub fn unstable_diffusion_1(input: &str) -> Ans1 {
    let parsed = parse(input);
    todo!("1")
}

pub fn unstable_diffusion_2(input: &str) -> Ans2 {
    let parsed = parse(input);
    todo!("2")
}

#[derive(Debug)]
struct Parsed {
    field: Vec<Vec<Tile>>,
}

#[derive(Debug)]
enum Tile {
    Vacant,
    Elf,
}

fn parse(str: &str) -> Parsed {
    let field = str
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => Tile::Vacant,
                    '#' => Tile::Elf,
                    ch => panic!("Unexpected char '{ch}"),
                })
                .collect_vec()
        })
        .collect_vec();
    Parsed { field }
}

// If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step.
// If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
// If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step.
// If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes moving east one step.
// If two or more Elves propose moving to the same position, none of those Elves move.
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

    const INPUT: &str = r#"
.....
..##.
..#..
.....
..##.
.....
    "#;

    #[test]
    fn parse_test() {
        let parsed = parse(INPUT.trim());
        dbg!(&parsed);
    }

    #[test]
    fn test_1() {
        let expected = todo!();
        let ans = unstable_diffusion_1(INPUT.trim());
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_2() {
        let expected = todo!();
        let ans = unstable_diffusion_2(INPUT.trim());
        assert_eq!(ans, expected);
    }
}
