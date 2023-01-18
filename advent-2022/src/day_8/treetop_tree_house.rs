use advent_2022_rs::get_input_str;
use itertools::Itertools;
use std::collections::HashSet;
use std::iter;

// https://adventofcode.com/2022/day/8

type Ans1 = usize;
type Ans2 = usize;

pub fn treetop_tree_house_1(input: &str) -> Ans1 {
    let field = parse(input).0;
    let i_max = field.len();
    let j_max = field[0].len();

    let horiz_interior_visible: HashSet<_> = get_horiz_visible(&field).collect();
    let vert_interior_visible: HashSet<_> = get_horiz_visible(&transpose(&field))
        .map(|(j, i)| (i, j))
        .collect();

    let interior_visible = horiz_interior_visible.union(&vert_interior_visible);
    let edge_visible_count = i_max * 2 + j_max * 2 - 4;
    edge_visible_count + interior_visible.count()
}

fn get_horiz_visible(field: &Vec<Vec<u8>>) -> impl Iterator<Item = (usize, usize)> {
    let mut interior_visible = HashSet::new();
    let (i_begin, i_end) = (1, field.len() - 1);
    let (j_begin, j_end) = (1, field[0].len() - 1);

    for i in i_begin..i_end {
        let mut max = 0_u8;
        for j in j_begin..j_end {
            max = max.max(field[i][j - 1]);
            if field[i][j] > max {
                interior_visible.insert((i, j));
            }
        }

        max = 0_u8;
        for j in (j_begin..j_end).rev() {
            max = max.max(field[i][j + 1]);
            if field[i][j] > max {
                interior_visible.insert((i, j));
            }
        }
    }
    interior_visible.into_iter()
}

fn transpose<T: Copy>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let i_max = matrix.len();
    let j_max = matrix[0].len();
    (0..j_max)
        .map(|j| (0..i_max).map(|i| matrix[i][j]).collect())
        .collect()
}

pub fn treetop_tree_house_2(input: &str) -> Ans2 {
    let parsed = parse(input);
    todo!("2")
}

#[derive(Debug)]
struct Parsed(Vec<Vec<u8>>);

fn parse(str: &str) -> Parsed {
    let vec_vec = str
        .split_whitespace()
        .map(|str| {
            str.chars()
                .map(|ch| ch.to_digit(10).expect("Parsed digit") as u8)
                .collect()
        })
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
