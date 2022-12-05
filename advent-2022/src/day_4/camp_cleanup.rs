use advent_2022_rs::get_file_str;
use itertools::Itertools;
use std::str::FromStr;

pub fn camp_cleanup(input: &str) -> usize {
    let parsed = parse(input);
    parsed
        .iter()
        .filter(|&(l, r)| l.0 >= r.0 && l.1 <= r.1 || l.0 <= r.0 && l.1 >= r.1)
        .count()
}

fn parse(str: &str) -> Vec<((i32, i32), (i32, i32))> {
    str.lines()
        .map(|l: &str| {
            l.split(',')
                .map(|p: &str| {
                    p.split('-')
                        .map(str::parse)
                        .map(Result::ok)
                        .flatten()
                        .collect_tuple::<(_, _)>()
                        .expect("Parse error!")
                })
                .collect_tuple::<(_, _)>()
                .expect("Parse error!")
        })
        .collect_vec()
}

fn main() {
    let str = get_file_str("day_4/input");
    let ans = camp_cleanup(&str);
    println!("Part 1: {ans}");
}

#[test]
fn camp_cleanup_test() {
    let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
    dbg!(input);
    let ans = camp_cleanup(input);
    assert_eq!(ans, 2);
}
