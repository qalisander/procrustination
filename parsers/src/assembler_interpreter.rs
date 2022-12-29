use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Pointer, Write};
use std::str::FromStr;
use thiserror::Error;
use AsmErr::*;
use Instr::*;
use Val::Num;

//NOTE: https://www.codewars.com/kata/58e61f3d8ff24f774400002c/train/rust

// TODO: use InstrType enum with indication of instruction line
// TODO: replace string with &str and reference to original string
#[derive(Debug, PartialEq, Clone)]
enum Instr {
    /// mov x, y - copy y (either an integer or the value of a register) into register x.
    Mov(Reg, Val),
    /// inc x - increase the content of register x by one.
    Inc(Reg),
    /// dec x - decrease the content of register x by one.
    Dec(Reg),
    /// add x, y - add the content of the register x with y (either an integer or the value of a register) and stores the result in x (i.e. register[x] += y).
    Add(Reg, Val),
    /// sub x, y - subtract y (either an integer or the value of a register) from the register x and stores the result in x (i.e. register[x] -= y).
    Sub(Reg, Val),
    /// mul x, y - same with multiply (i.e. register[x] *= y).
    Mul(Reg, Val),
    /// div x, y - same with integer division (i.e. register[x] /= y).
    Div(Reg, Val),
    /// label: - define a label position (label = identifier + ":", an identifier being a string that does not match any other command). Jump commands and call are aimed to these labels positions in the program.
    Lbl(Lbl),
    /// jmp lbl - jumps to the label lbl.
    Jmp(Lbl),
    /// cmp x, y - compares x (either an integer or the value of a register) and y (either an integer or the value of a register). The result is used in the conditional jumps (jne, je, jge, jg, jle and jl)
    Cmp(Val, Val),
    /// jne lbl - jump to the label lbl if the values of the previous cmp command were not equal.
    Jne(Lbl),
    /// je lbl - jump to the label lbl if the values of the previous cmp command were equal.
    Je(Lbl),
    /// jge lbl - jump to the label lbl if x was greater or equal than y in the previous cmp command.
    Jge(Lbl),
    /// jg lbl - jump to the label lbl if x was greater than y in the previous cmp command.
    Jg(Lbl),
    /// jle lbl - jump to the label lbl if x was less or equal than y in the previous cmp command.
    Jle(Lbl),
    /// jl lbl - jump to the label lbl if x was less than y in the previous cmp command.
    Jl(Lbl),
    /// call lbl - call to the subroutine identified by lbl. When a ret is found in a subroutine, the instruction pointer should return to the instruction next to this call command.
    Call(Lbl),
    /// ret - when a ret is found in a subroutine, the instruction pointer should return to the instruction that called the current function.
    Ret,
    /// msg 'Register: ', x - this instruction stores the output of the program. It may contain text strings (delimited by single quotes) and registers. The number of arguments isn't limited and will vary, depending on the program.
    Msg(Vec<MsgArg>),
    /// end - this instruction indicates that the program ends correctly, so the stored output is returned (if the program terminates without this instruction it should return the default output: see below).
    End,
}

#[derive(Debug, PartialEq, Clone)]
enum MsgArg {
    Reg(Reg),
    Txt(String),
}

impl Display for MsgArg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{0}",
            match self {
                MsgArg::Reg(reg) => &reg.0,
                MsgArg::Txt(str) => str,
            }
        )
    }
}

impl FromStr for MsgArg {
    type Err = AsmErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')) {
            None => Ok(MsgArg::Reg(s.parse()?)),
            Some(text) => Ok(MsgArg::Txt(text.to_string())),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Lbl(String);

impl Display for Lbl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{0}", self.0)
    }
}

impl FromStr for Lbl {
    type Err = AsmErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Lbl(s.to_string()))
    }
}

// TODO: validate name of register when new register is created
#[derive(Debug, Hash, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Reg(String);

impl Display for Reg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", &self.0)
    }
}

impl FromStr for Reg {
    type Err = AsmErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Reg(s.to_string()))
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Val {
    Reg(Reg),
    Num(Number),
}

type Number = i32;

impl FromStr for Val {
    type Err = AsmErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<Number>() {
            Ok(num) => Ok(Num(num)),
            Err(_) => Ok(Val::Reg(s.parse()?)),
        }
    }
}

#[derive(Error, Debug, PartialEq)]
enum AsmErr {
    #[error("Invalid instruction!")]
    InvalidInstr,
    #[error("Missing character '{0}'!")]
    CharacterExpected(char),
    #[error("Register does not exist '{0}'!")]
    NonExistentRegister(Reg),
    #[error("Label does not exist '{0}'!")]
    NonExistentLabel(Lbl),
    #[error("Invalid return 'ret' from call!")]
    InvalidRet,
    #[error(transparent)]
    FmtErr(#[from] std::fmt::Error),
}

#[derive(Debug)]
pub struct AssemblerInterpreter {
    instructions: Vec<Instr>,
    registers: Registers,
    labels: Labels,
    stack: Vec<usize>,
}

#[derive(Debug)]
struct Registers(HashMap<Reg, Number>);

impl Registers {
    fn get(&self, val: &Val) -> Result<Number, AsmErr> {
        match val {
            Val::Reg(reg) => self
                .0
                .get(reg)
                .cloned()
                .ok_or(NonExistentRegister(reg.clone())),
            Num(num) => Ok(*num),
        }
    }

    fn get_mut(&mut self, reg: &Reg) -> Result<&mut Number, AsmErr> {
        self.0
            .get_mut(reg)
            .ok_or_else(|| NonExistentRegister(reg.clone()))
    }

    fn insert(&mut self, reg: &Reg, num: Number) {
        self.0.insert(reg.clone(), num);
    }
}

#[derive(Debug)]
struct Labels(HashMap<Lbl, usize>);

impl Labels {
    fn get(&self, lbl: &Lbl) -> Result<usize, AsmErr> {
        self.0
            .get(lbl)
            .cloned()
            .ok_or(NonExistentLabel(lbl.clone()))
    }
}

impl AssemblerInterpreter {
    pub fn interpret(input: &str) -> Option<String> {
        // Collect parsed lines. We have to know functions to call in advance
        let instructions: Vec<Instr> = Self::scan(input)
            .into_iter()
            .try_collect()
            .expect("Error while scanning!");

        let labels = instructions
            .iter()
            .enumerate()
            .filter_map(|(i, instr)| match instr {
                Instr::Lbl(lbl) => Some((lbl.clone(), i)),
                _ => None,
            })
            .collect();

        let mut interpreter = AssemblerInterpreter {
            instructions,
            labels: Labels(labels),
            registers: Registers(HashMap::new()),
            stack: vec![],
        };

        dbg!(&interpreter);
        interpreter
            .interpret_instr()
            .expect("Error while interpreting instructions!")
    }

    /// Main processor emulation logic. With call stack, label storage and registers
    fn interpret_instr(&mut self) -> Result<Option<String>, AsmErr> {
        let mut output = String::new();
        let mut prev_ord = None;
        let mut i: usize = 0;
        loop {
            if i >= self.instructions.len() {
                break Ok(None);
            }

            let instr = &self.instructions[i];
            match instr {
                Mov(reg, val) => {
                    let val = self.registers.get(val)?;
                    self.registers.insert(reg, val);
                }
                Inc(reg) => {
                    let val = self.registers.get_mut(reg)?;
                    *val += 1;
                }
                Dec(reg) => {
                    let val = self.registers.get_mut(reg)?;
                    *val -= 1;
                }
                Add(reg, val) => {
                    let r_val = self.registers.get(val)?;
                    let l_val = self.registers.get_mut(reg)?;
                    *l_val += r_val;
                }
                Sub(reg, val) => {
                    let r_val = self.registers.get(val)?;
                    let l_val = self.registers.get_mut(reg)?;
                    *l_val -= r_val;
                }
                Mul(reg, val) => {
                    let r_val = self.registers.get(val)?;
                    let l_val = self.registers.get_mut(reg)?;
                    *l_val *= r_val;
                }
                Div(reg, val) => {
                    let r_val = self.registers.get(val)?;
                    let l_val = self.registers.get_mut(reg)?;
                    *l_val /= r_val;
                }
                Instr::Lbl(_) => {}
                Jmp(lbl) => i = self.labels.get(lbl)?,
                Cmp(val0, val1) => {
                    let val0 = self.registers.get(val0)?;
                    let val1 = self.registers.get(val1)?;
                    prev_ord = Some(val0.cmp(&val1));
                }
                Jne(lbl) => {
                    if !matches!(prev_ord.take(), Some(Ordering::Equal)) {
                        i = self.labels.get(lbl)?
                    }
                }
                Je(lbl) => {
                    if matches!(prev_ord.take(), Some(Ordering::Equal)) {
                        i = self.labels.get(lbl)?;
                    }
                }
                Jge(lbl) => {
                    if matches!(
                        prev_ord.take(),
                        Some(Ordering::Greater) | Some(Ordering::Equal)
                    ) {
                        i = self.labels.get(lbl)?;
                    }
                }
                Jg(lbl) => {
                    if matches!(prev_ord.take(), Some(Ordering::Greater)) {
                        i = self.labels.get(lbl)?;
                    }
                }
                Jle(lbl) => {
                    if matches!(
                        prev_ord.take(),
                        Some(Ordering::Less) | Some(Ordering::Equal)
                    ) {
                        i = self.labels.get(lbl)?;
                    }
                }
                Jl(lbl) => {
                    if matches!(prev_ord.take(), Some(Ordering::Less)) {
                        i = self.labels.get(lbl)?;
                    }
                }
                Call(lbl) => {
                    self.stack.push(i);
                    i = self.labels.get(lbl)?;
                }
                Ret => {
                    i = self.stack.pop().ok_or(InvalidRet)?;
                }
                Msg(args) => {
                    let args: Vec<String> = args
                        .iter()
                        .map(|arg| match arg {
                            MsgArg::Reg(reg) => self
                                .registers
                                .get(&Val::Reg(reg.clone()))
                                .map(|num| num.to_string()),
                            MsgArg::Txt(str) => Ok(str.clone()),
                        })
                        .try_collect()?;
                    let msg = args.join("");
//                    let result = File::open("src/file.txt").into()?;
                    write!(&mut output, "{msg}")?;
                }

                End => break Ok(Some(output)),
            }
            i += 1;
        }
    }

    fn scan(input: &str) -> Vec<Result<Instr, AsmErr>> {
        input
            .lines()
            .map(Self::scan_line)
            .filter_map_ok(|instr| instr)
            .collect_vec()
    }

    fn scan_line(line: &str) -> Result<Option<Instr>, AsmErr> {
        /// Parse instruction with one argument
        fn one_arg<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<&'a str, AsmErr> {
            match tokens.next() {
                None => Err(InvalidInstr),
                Some(arg) => Ok(arg),
            }
        }

        /// Parse instruction with two arguments
        fn two_arg<'a>(
            tokens: &mut impl Iterator<Item = &'a str>,
        ) -> Result<(&'a str, &'a str), AsmErr> {
            match one_arg(tokens)?.strip_suffix(',') {
                None => Err(CharacterExpected(',')),
                Some(arg0) => {
                    let arg1 = one_arg(tokens)?;
                    Ok((arg0, arg1))
                }
            }
        }

        // Strip comments or whole string if it's a comment line
        let line = match line.split(';').next().map(str::trim) {
            None | Some("") => return Ok(None),
            Some(line) => line,
        };

        let mut tokens = line.split_whitespace();
        let instr = match tokens
            .next()
            .expect("There is at least one no whitespace character")
        {
            "mov" => {
                let (arg0, arg1) = two_arg(&mut tokens)?;
                Mov(arg0.parse()?, arg1.parse()?)
            }
            "inc" => Inc(one_arg(&mut tokens)?.parse()?),
            "dec" => Dec(one_arg(&mut tokens)?.parse()?),
            "add" => {
                let (arg0, arg1) = two_arg(&mut tokens)?;
                Add(arg0.parse()?, arg1.parse()?)
            }
            "sub" => {
                let (arg0, arg1) = two_arg(&mut tokens)?;
                Sub(arg0.parse()?, arg1.parse()?)
            }
            "mul" => {
                let (arg0, arg1) = two_arg(&mut tokens)?;
                Mul(arg0.parse()?, arg1.parse()?)
            }
            "div" => {
                let (arg0, arg1) = two_arg(&mut tokens)?;
                Div(arg0.parse()?, arg1.parse()?)
            }
            "jmp" => Jmp(one_arg(&mut tokens)?.parse()?),
            "cmp" => {
                let (arg0, arg1) = two_arg(&mut tokens)?;
                Cmp(arg0.parse()?, arg1.parse()?)
            }
            "jne" => Jne(one_arg(&mut tokens)?.parse()?),
            "je" => Je(one_arg(&mut tokens)?.parse()?),
            "jge" => Jge(one_arg(&mut tokens)?.parse()?),
            "jg" => Jg(one_arg(&mut tokens)?.parse()?),
            "jle" => Jle(one_arg(&mut tokens)?.parse()?),
            "jl" => Jl(one_arg(&mut tokens)?.parse()?),
            "call" => Call(one_arg(&mut tokens)?.parse()?),
            "ret" => Ret,
            "msg" => Msg(Self::parse_msg(line)?),
            "end" => End,
            label => {
                if let Some(label) = label.strip_suffix(':') {
                    Instr::Lbl(Lbl(label.to_string()))
                } else {
                    return Err(CharacterExpected(':'));
                }
            }
        };
        Ok(Some(instr))
    }

    /// Parse message instruction. Complex because of possible existence of spaces and commas inside ' ';
    fn parse_msg(line: &str) -> Result<Vec<MsgArg>, AsmErr> {
        let mut is_text = false;
        let args: Vec<MsgArg> = line
            .strip_prefix("msg")
            .expect("Msg already part of line")
            .chars()
            .batching(|chars| {
                let mut buffer = String::new();
                loop {
                    let ch = chars.next();
                    match ch {
                        None if buffer.is_empty() => break None,
                        None => {
                            break if is_text {
                                Some(Err(InvalidInstr))
                            } else {
                                Some(Ok(MsgArg::Reg(Reg(buffer))))
                            };
                        }
                        Some('\'') if buffer.is_empty() => {
                            is_text ^= true;
                        }
                        Some('\'') => {
                            let ans = if is_text {
                                Some(Ok(MsgArg::Txt(buffer)))
                            } else {
                                Some(Ok(MsgArg::Reg(Reg(buffer))))
                            };
                            is_text ^= true;
                            break ans;
                        }
                        Some(' ' | ',') if !is_text => {}
                        Some(ch) => buffer.push(ch),
                    }
                }
            })
            .try_collect()?;
        Ok(args)
    }
}

#[test]
fn scan_test_1() {
    let input = r"
; My first program
mov  a, 5
inc  a
call function
msg  '(5+1)/2 = ', a    ; output message
end

function:
    div  a, 2
    ret
";
    let expected_instructions = [
        Instr::Mov(Reg("a".to_string()), Val::Num(5)),
        Instr::Inc(Reg("a".to_string())),
        Instr::Call(Lbl("function".to_string())),
        Instr::Msg(vec![
            MsgArg::Txt("(5+1)/2 = ".to_string()),
            MsgArg::Reg(Reg("a".to_string())),
        ]),
        Instr::End,
        Instr::Lbl(Lbl("function".to_string())),
        Instr::Div(Reg("a".to_string()), Val::Num(2)),
        Instr::Ret,
    ]
    .map(|i| Ok(i));
    let instructions = AssemblerInterpreter::scan(input);
    itertools::assert_equal(instructions, expected_instructions)
}

#[test]
fn scan_test_2() {
    let input = r"
mov   a, 11           ; value1
mov   b, 3            ; value2
call  mod_func
msg   'mod(', a, ', ', b, ') = ', d        ; output
end

; Mod function
mod_func:
    mov   c, a        ; temp1
    div   c, b
    mul   c, b
    mov   d, a        ; temp2
    sub   d, c
    ret
";
    let expected_instructions = [
        Instr::Mov(Reg("a".to_string()), Val::Num(11)),
        Instr::Mov(Reg("b".to_string()), Val::Num(3)),
        Instr::Call(Lbl("mod_func".to_string())),
        Instr::Msg(vec![
            MsgArg::Txt("mod(".to_string()),
            MsgArg::Reg(Reg("a".to_string())),
            MsgArg::Txt(", ".to_string()),
            MsgArg::Reg(Reg("b".to_string())),
            MsgArg::Txt(") = ".to_string()),
            MsgArg::Reg(Reg("d".to_string())),
        ]),
        Instr::End,
        Instr::Lbl(Lbl("mod_func".to_string())),
        Instr::Mov(Reg("c".to_string()), Val::Reg(Reg("a".to_string()))),
        Instr::Div(Reg("c".to_string()), Val::Reg(Reg("b".to_string()))),
        Instr::Mul(Reg("c".to_string()), Val::Reg(Reg("b".to_string()))),
        Instr::Mov(Reg("d".to_string()), Val::Reg(Reg("a".to_string()))),
        Instr::Sub(Reg("d".to_string()), Val::Reg(Reg("c".to_string()))),
        Instr::Ret,
    ]
    .map(|i| Ok(i));
    let instructions = AssemblerInterpreter::scan(input);
    itertools::assert_equal(instructions, expected_instructions)
}

#[test]
fn simple_test() {
    let simple_programs = &[
        r"
; My first program
mov  a, 5
inc  a
call function
msg  '(5+1)/2 = ', a    ; output message
end

function:
    div  a, 2
    ret
",
        r"
mov   a, 5
mov   b, a
mov   c, a
call  proc_fact
call  print
end

proc_fact:
    dec   b
    mul   c, b
    cmp   b, 1
    jne   proc_fact
    ret

print:
    msg   a, '! = ', c ; output text
    ret
",
        r"
mov   a, 8            ; value
mov   b, 0            ; next
mov   c, 0            ; counter
mov   d, 0            ; first
mov   e, 1            ; second
call  proc_fib
call  print
end

proc_fib:
    cmp   c, 2
    jl    func_0
    mov   b, d
    add   b, e
    mov   d, e
    mov   e, b
    inc   c
    cmp   c, a
    jle   proc_fib
    ret

func_0:
    mov   b, c
    inc   c
    jmp   proc_fib

print:
    msg   'Term ', a, ' of Fibonacci series is: ', b        ; output text
    ret
", ////////////////////////
        r"
mov   a, 11           ; value1
mov   b, 3            ; value2
call  mod_func
msg   'mod(', a, ', ', b, ') = ', d        ; output
end

; Mod function
mod_func:
    mov   c, a        ; temp1
    div   c, b
    mul   c, b
    mov   d, a        ; temp2
    sub   d, c
    ret
",
        r"
mov   a, 81         ; value1
mov   b, 153        ; value2
call  init
call  proc_gcd
call  print
end

proc_gcd:
    cmp   c, d
    jne   loop
    ret

loop:
    cmp   c, d
    jg    a_bigger
    jmp   b_bigger

a_bigger:
    sub   c, d
    jmp   proc_gcd

b_bigger:
    sub   d, c
    jmp   proc_gcd

init:
    cmp   a, 0
    jl    a_abs
    cmp   b, 0
    jl    b_abs
    mov   c, a            ; temp1
    mov   d, b            ; temp2
    ret

a_abs:
    mul   a, -1
    jmp   init

b_abs:
    mul   b, -1
    jmp   init

print:
    msg   'gcd(', a, ', ', b, ') = ', c
    ret
",
        r"
call  func1
call  print
end

func1:
    call  func2
    ret

func2:
    ret

print:
    msg 'This program should return null'
",
        r"
mov   a, 2            ; value1
mov   b, 10           ; value2
mov   c, a            ; temp1
mov   d, b            ; temp2
call  proc_func
call  print
end

proc_func:
    cmp   d, 1
    je    continue
    mul   c, a
    dec   d
    call  proc_func

continue:
    ret

print:
    msg a, '^', b, ' = ', c
    ret
",
    ];

    let expected = &[
        Some(String::from("(5+1)/2 = 3")),
        Some(String::from("5! = 120")),
        Some(String::from("Term 8 of Fibonacci series is: 21")),
        Some(String::from("mod(11, 3) = 2")),
        Some(String::from("gcd(81, 153) = 9")),
        None,
        Some(String::from("2^10 = 1024")),
    ];

    for (i, (prg, exp)) in simple_programs.iter().zip(expected).enumerate() {
        let actual = AssemblerInterpreter::interpret(*prg);
        assert_eq!(actual, *exp, "Test {} failed!", i);
    }
}
