use advent_2022::get_input_str;
use derive_more::{Add, Deref, Mul};
use itertools::iproduct;
use std::collections::HashSet;
use std::ops::Index;

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
    let field = parse(input);
    field
        .coord_iter()
        .map(|x| field.get_scenic_score(x))
        .max()
        .expect("Not empty field")
}

#[derive(Deref, Debug)]
struct Field(Vec<Vec<u8>>);

impl Field {
    fn in_bounds(&self, coord: Coord) -> bool {
        0_i32 <= coord.0 && coord.0 < self.i_max() && 0_i32 <= coord.1 && coord.1 < self.j_max()
    }

    fn i_max(&self) -> i32 {
        self.0.len() as i32
    }

    fn j_max(&self) -> i32 {
        self.0[0].len() as i32
    }

    fn coord_iter(&self) -> impl Iterator<Item = Coord> + '_ {
        iproduct!((0..self.i_max()), (0..self.j_max())).map(Coord::from)
    }

    fn get_scenic_score(&self, x: Coord) -> usize {
        let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        deltas
            .into_iter()
            .map(Coord::from)
            .map(|delta| {
                let mut score = 0;
                let mut curr_x = x + delta;
                while self.in_bounds(curr_x) {
                    score += 1;
                    if self[curr_x] >= self[x] {
                        break;
                    }
                    curr_x = curr_x + delta
                }
                score
            })
            .product()
    }
}

impl Index<Coord> for Field {
    type Output = u8;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.0[index.0 as usize][index.1 as usize]
    }
}

#[derive(Add, Copy, Clone, Debug)]
struct Coord(i32, i32);

impl<T: Into<i32>> From<(T, T)> for Coord {
    fn from(value: (T, T)) -> Self {
        Coord(value.0.into(), value.1.into())
    }
}

fn parse(str: &str) -> Field {
    let vec_vec = str
        .split_whitespace()
        .map(|str| {
            str.chars()
                .map(|ch| ch.to_digit(10).expect("Parsed digit") as u8)
                .collect()
        })
        .collect();
    Field(vec_vec)
}

fn main() {
    let str = get_input_str(file!());
    let ans = treetop_tree_house_1(&str);
    assert_eq!(1546, ans);
    println!("Part 1: {ans}");
    let ans = treetop_tree_house_2(&str);
    assert_eq!(519064, ans);
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
        let expected = 8;
        let ans = treetop_tree_house_2(get_input());
        assert_eq!(ans, expected);
    }
}
