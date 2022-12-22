use crate::Stmt::{Bin, Un};
use advent_2022_rs::get_input_str;
use itertools::Itertools;

// https://adventofcode.com/2022/day/21

type Ans = i32;
type Ans1 = Ans;
type Ans2 = Ans;

pub fn monkey_math_1(input: &str) -> Ans1 {
    let parsed = parse(input);
    todo!("1")
}

pub fn monkey_math_2(input: &str) -> Ans2 {
    let parsed = parse(input);
    todo!("2")
}

#[derive(Debug)]
struct Parsed<'a>(Vec<Stmt<'a>>);

#[derive(Debug)]
enum Stmt<'a> {
    Bin {
        val: Var<'a>,
        var0: Var<'a>,
        op: Op,
        var1: Var<'a>,
    },
    Un {
        val: Var<'a>,
        num: Ans,
    },
}

#[derive(Debug)]
struct Var<'a>(&'a str);

#[derive(Debug)]
struct Op(char);

fn parse(str: &str) -> Parsed {
    let vec = str
        .lines()
        .map(|line| {
            if let Some((val, num)) = line.split(' ').collect_tuple::<(_, _)>() {
                Un {
                    val: Var(val.strip_suffix(':').expect("Has suffix ':'")),
                    num: num.parse().expect("Num is number"),
                }
            } else if let Some((val, var0, op, var1)) =
                line.split(' ').collect_tuple::<(_, _, _, _)>()
            {
                Bin {
                    val: Var(val.strip_suffix(':').expect("Has suffix ':'")),
                    var0: Var(var0),
                    op: Op(op.parse().expect("Op is char")),
                    var1: Var(var1),
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
        let expected = 30;
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
