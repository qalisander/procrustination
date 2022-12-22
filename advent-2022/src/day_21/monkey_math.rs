use crate::Val::*;
use advent_2022_rs::get_input_str;
use itertools::Itertools;
use std::collections::HashMap;

// https://adventofcode.com/2022/day/21

type Ans = i64;
type Ans1 = Ans;
type Ans2 = Ans;

pub fn monkey_math_1(input: &str) -> Ans1 {
    fn calc_rec(var: Var, var_to_val: &HashMap<Var, &Val>) -> Ans {
        let val = *var_to_val
            .get(var)
            .unwrap_or_else(|| panic!("Var '{var}' should exist"));
        match *val {
            Bin { var0, op, var1 } => {
                calc(calc_rec(var0, var_to_val), op, calc_rec(var1, var_to_val))
            }
            Un(ans) => ans,
        }
    }

    let parsed = parse(input).0;
    let var_to_val: HashMap<_, _> = parsed.iter().map(|stmt| (stmt.lval, &stmt.rval)).collect();

    calc_rec("root", &var_to_val)
}

fn calc(val0: Ans, op: Op, val1: Ans) -> Ans {
    match op {
        '+' => val0 + val1,
        '-' => val0 - val1,
        '*' => val0 * val1,
        '/' => val0 / val1,
        op => panic!("Invalid operation '{op}'"),
    }
}

pub fn monkey_math_2(input: &str) -> Ans2 {
    let parsed = parse(input);
    todo!("2")
}

#[derive(Debug)]
struct Parsed<'a>(Vec<Stmt<'a>>);

#[derive(Debug)]
struct Stmt<'a> {
    lval: Var<'a>,
    rval: Val<'a>,
}

#[derive(Debug)]
enum Val<'a> {
    Bin {
        var0: Var<'a>,
        op: Op,
        var1: Var<'a>,
    },
    Un(Ans),
}

type Var<'a> = &'a str;

type Op = char;

fn parse(str: &str) -> Parsed {
    let vec = str
        .lines()
        .map(|line| {
            if let Some((lval, num)) = line.split(' ').collect_tuple::<(_, _)>() {
                Stmt {
                    lval: lval.strip_suffix(':').expect("Has suffix ':'"),
                    rval: Un(num.parse().expect("Num is number")),
                }
            } else if let Some((val, var0, op, var1)) =
                line.split(' ').collect_tuple::<(_, _, _, _)>()
            {
                Stmt {
                    lval: val.strip_suffix(':').expect("Has suffix ':'"),
                    rval: Bin {
                        var0,
                        op: op.parse().expect("Op is char"),
                        var1,
                    },
                }
            } else {
                panic!("Invalid string");
            }
        })
        .collect_vec();
    Parsed(vec)
}

fn main() {
    let str = get_input_str(file!());
    let ans = monkey_math_1(&str);
    println!("Part 1: {ans}");
    let ans = monkey_math_2(&str);
    println!("Part 2: {ans}");
}

#[cfg(test)]
mod tests {
    use crate::{monkey_math_1, monkey_math_2, parse};

    const INPUT: &str = r#"
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
    "#;

    #[test]
    fn parse_test() {
        let parsed = parse(INPUT.trim());
        dbg!(&parsed);
    }

    #[test]
    fn test_1() {
        let expected = 152;
        let ans = monkey_math_1(INPUT.trim());
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_2() {
        let expected = todo!();
        let ans = monkey_math_2(INPUT.trim());
        assert_eq!(ans, expected);
    }
}
