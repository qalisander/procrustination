use advent_2022_rs::get_input_str;
use itertools::Itertools;

pub fn camp_cleanup_1(input: &str) -> usize {
    let parsed = parse(input);
    parsed
        .iter()
        .filter(|&(a, b)| a.0 >= b.0 && a.1 <= b.1 || a.0 <= b.0 && a.1 >= b.1)
        .count()
}

// .234.....  2-4
// .....678.  6-8

// .23......  2-3
// ...45....  4-5

// ....567..  5-7
// ......789  7-9

// ....567..  5-7
// ...789...  7-9

// .2345678.  2-8
// ..34567..  3-7

// .....6...  6-6
// ...456...  4-6

// .23456...  2-6
// ...45678.  4-8

pub fn camp_cleanup_2(input: &str) -> usize {
    let parsed = parse(input);
    let vec = parsed
        .iter()
        .filter(|&(a, b)| {
            b.0 <= a.0 && a.0 <= b.1
                || b.0 <= a.1 && a.1 <= b.1
                || a.0 <= b.0 && b.0 <= a.1
                || a.0 <= b.1 && b.1 <= a.1
        })
        .collect_vec();
    vec.len()
}

fn parse(str: &str) -> Vec<((i32, i32), (i32, i32))> {
    str.lines()
        .map(|l| {
            l.split(',')
                .map(|p: &str| {
                    p.split('-')
                        .map(str::parse)
                        .filter_map(Result::ok)
                        .collect_tuple::<(_, _)>()
                        .expect("Parse error!")
                })
                .collect_tuple::<(_, _)>()
                .expect("Parse error!")
        })
        .collect_vec()
}

fn main() {
    let str = get_input_str(file!());
    let ans = camp_cleanup_1(&str);
    println!("Part 1: {ans}");
    let ans = camp_cleanup_2(&str);
    println!("Part 2: {ans}");
}

#[test]
fn camp_cleanup_test() {
    let input = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
    "#;
    dbg!(input);
    let ans = camp_cleanup_1(input);
    assert_eq!(ans, 2);
    let ans = camp_cleanup_2(input);
    assert_eq!(ans, 4)
}
