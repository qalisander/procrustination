use crate::Val::*;
use advent_2022_rs::get_input_str;
use itertools::{Either, Itertools};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

// https://adventofcode.com/2022/day/21

type Ans = i64;
type Ans1 = Ans;
type Ans2 = Ans;

pub fn monkey_math_1(input: &str) -> Ans1 {
    let parsed = parse(input).0;
    let var_to_val: HashMap<_, _> = parsed.iter().map(|stmt| (stmt.lval, &stmt.rval)).collect();

    calc_rec("root", &var_to_val)
}

fn calc_rec(var: Var, var_to_val: &HashMap<Var, &Val>) -> Ans {
    let val = *var_to_val
        .get(var)
        .unwrap_or_else(|| panic!("Var '{var}' should exist"));
    match *val {
        Bin {
            left: var0,
            op,
            right: var1,
        } => calc(calc_rec(var0, var_to_val), op, calc_rec(var1, var_to_val)),
        Un(ans) => ans,
    }
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
    let mut stmts = parse(input).0;
    let mut var_info: HashMap<Var, VarInfo> = HashMap::new();
    for (i, stmt) in stmts.iter().enumerate() {
        if let Bin {left, op, right}  = stmt.rval {
            var_info.entry(left).or_default().left = Some(i);
            var_info.entry(right).or_default().right = Some(i);
        }
    }

    revert_stmts_rec("humn", &var_info, &mut stmts);

    let var_to_val: HashMap<_, _> = stmts.iter().map(|stmt| (stmt.lval, &stmt.rval)).collect();
    calc_rec("humn", &var_to_val)
}

/// Reference to index in stmt's vec
/// where var exist
#[derive(Default)]
struct VarInfo{
    left: Option<usize>,
    right: Option<usize>,
}

fn revert_stmts_rec(var: Var, vars: &HashMap<Var, VarInfo>, stmts: &mut Vec<Stmt>){
    let var_info = vars.get(var).unwrap_or_else(|| panic!("{var} is in input"));
    if let Some(i) = var_info.left {
        revert_stmts_rec(stmts[i].lval, vars, stmts);
        stmts[i].revert_mut(Arg::Left);
    }
    if let Some(i) = var_info.right {
        revert_stmts_rec(stmts[i].lval, vars, stmts);
        stmts[i].revert_mut(Arg::Right);
    }
}


#[derive(Debug)]
struct Parsed<'a>(Vec<Stmt<'a>>);

#[derive(Debug, Copy, Clone)]
struct Stmt<'a> {
    lval: Var<'a>,
    rval: Val<'a>,
}

impl<'a> Display for Stmt<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.rval {
            Bin {left, op, right} => {
                write!(f, "{}: {} {} {}", self.lval, left, op, right)?; }
            Un(num) => {
                write!(f, "{}: {}", self.lval, num)?;
            }
        }
        Ok(())
    }
}

enum Arg{
    Left,
    Right,
}

impl<'a> Stmt<'a> {
    pub fn revert(&self, new_root: Arg) -> Stmt {
        match self.rval {
            Bin { left, op, right } => match new_root {
                Arg::Left => Stmt {
                    lval: left,
                    rval: Bin {
                        left: self.lval,
                        op: revert_op(op),
                        right: right,
                    },
                },
                Arg::Right => {
                    Stmt{
                        lval: right,
                        rval: if is_commutative(op){
                            Bin {
                                left: self.lval,
                                op: revert_op(op),
                                right: left,
                            }
                        } else {
                            Bin {
                                left: left,
                                op: revert_op(op),
                                right: self.lval,
                            }
                        }
                    }
                },
            },
            Un(num) => *self,
        }
    }

    pub fn revert_mut(&mut self, new_root: Arg){
        *self = self.revert(new_root)
    }
}


fn is_commutative(ch: Op) -> bool{
    matches!(ch, '+' | '*')
}

fn revert_op(ch: Op) -> Op{
    match ch {
        '-' => '+',
        '+' => '-',
        '/' => '*',
        '*' => '/',
        op => panic!("Invalid operation '{op}'"),
    }
}

#[derive(Debug, Copy, Clone)]
enum Val<'a> {
    Bin {
        left: Var<'a>,
        op: Op,
        right: Var<'a>,
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
            } else if let Some((val, left, op, right)) =
                line.split(' ').collect_tuple::<(_, _, _, _)>()
            {
                Stmt {
                    lval: val.strip_suffix(':').expect("Has suffix ':'"),
                    rval: Bin {
                        left,
                        op: op.parse().expect("Op is char"),
                        right,
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
    use super::*;

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

//    root: pppw(150) + sjmn(150) // root = 0
//    dbpl: 5
//    cczh: sllz(4) + lgvd(596) // 600
//    zczc: 2
//    ptdq: humn(301) - dvpt(3) // 298
//    dvpt: 3
//    lfqf: 4
//    humn: 5 //301
//    ljgn: 2
//    sjmn: drzm(30) * dbpl(5)
//    sllz: 4
//    pppw: cczh(600) / lfqf(4) // 150
//    lgvd: ljgn(2) * ptdq(298) // 596
//    drzm: hmdt(32) - zczc(2) // 30
//    hmdt: 32

    #[test]
    fn rev_stmt_test() {
        let parsed = parse("pppw: cczh / lfqf").0.into_iter().next().unwrap();
        let reverted = parsed.revert(Arg::Right);
        assert_eq!(reverted.to_string(), "lfqf: cczh * pppw");
        let reverted = parsed.revert(Arg::Left);
        assert_eq!(reverted.to_string(), "cczh: pppw * lfqf");

        let parsed = parse("root: pppw + sjmn").0.into_iter().next().unwrap();
        let reverted = parsed.revert(Arg::Right);
        assert_eq!(reverted.to_string(), "sjmn: root - pppw");
        let reverted = parsed.revert(Arg::Left);
        assert_eq!(reverted.to_string(), "pppw: root - sjmn");
    }

    #[test]
    fn test_2() {
        let expected = 301;
        let ans = monkey_math_2(INPUT.trim());
        assert_eq!(ans, expected);
    }
}
