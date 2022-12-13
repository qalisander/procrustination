use crate::assembler_interpreter::AsmErr::*;
use crate::assembler_interpreter::Instr::*;
use crate::assembler_interpreter::Val::Num;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Pointer, Write};
use std::str::FromStr;
use thiserror::Error;

//NOTE: https://www.codewars.com/kata/58e61f3d8ff24f774400002c/train/rust

// TODO: use InstrType enum
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
        write!(f, "{0}", match self {
            MsgArg::Reg(reg) => &reg.0,
            MsgArg::Txt(str) => str,
        })
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

impl FromStr for Lbl {
    type Err = AsmErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Lbl(s.to_string()))
    }
}

// TODO: validate name of register prlly when new register is created
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
enum Val {
    Reg(Reg),
    Num(RegVal),
}

type RegVal = i32;

impl FromStr for Val {
    type Err = AsmErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<RegVal>() {
            Ok(num) => Ok(Num(num)),
            Err(_) => Ok(Val::Reg(s.parse()?)),
        }
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum AsmErr {
    #[error("Invalid instruction!")]
    InvalidInstr,
    #[error("Missing character '{0}'!")]
    CharacterExpected(char),
    #[error("Register does not exist '{0}'!")]
    NonExistentRegister(Reg),
}

pub struct AssemblerInterpreter {
    instructions: Vec<Instr>,
    registers: HashMap<Reg, RegVal>,
    labels: HashMap<Lbl, usize>,
}

impl AssemblerInterpreter {
    pub fn interpret(input: &str) -> Option<String> {
        // NOTE: Collect parsed lines. We have to know functions in advance
        let instructions: Vec<Instr> = Self::scan(input)
            .into_iter()
            .try_collect()
            .expect("Error while scanning!");

        let registers = HashMap::new();
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
            registers,
            labels,
        };

        interpreter
            .interpret_instr()
            .expect("Error while interpreting instructions!")
    }

    fn interpret_instr(&mut self) -> Result<Option<String>, AsmErr> {
        let mut output = String::new();
        let mut i: usize = 0;
        return loop {
            if i >= self.instructions.len() {
                break Ok(None)
            }
            
            let instr = &self.instructions[i];
            match instr {
                Mov(reg, val) => {
                    let val = match val {
                        Val::Reg(reg1) => self
                            .registers
                            .get(reg1)
                            .ok_or(NonExistentRegister(reg1.clone()))?,
                        Num(num) => num,
                    };
                    self.registers.insert(reg.clone(), *val);
                }
                Inc(reg) => {
                    let entry = self.registers.get_mut(reg).ok_or(NonExistentRegister(reg.clone()))?;
                    *entry += 1;
                }
                Dec(reg) => {
                    let entry = self.registers.get_mut(reg).ok_or(NonExistentRegister(reg.clone()))?;
                    *entry -= 1;
                }
                Add(reg, val) => {}
                Sub(reg, val) => {}
                Mul(reg, val) => {}
                Div(reg, val) => {}
//                Instr::Lbl(_) => {}
                Jmp(lbl) => {}
                Cmp(val0, val1) => {}
                Jne(lbl) => {}
                Je(lbl) => {}
                Jge(lbl) => {}
                Jg(lbl) => {}
                Jle(lbl) => {}
                Jl(lbl) => {}
                Call(lbl) => {}
                Ret => {
                    // TODO: add stack
                }
                Msg(args) => {
                    let msg = args.iter().map(MsgArg::to_string).join("");
                    writeln!(&mut output, "{msg}");
                }
                End => break Ok(Some(output)),
                _ => (),
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
        fn one_arg<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<&'a str, AsmErr> {
            // BUG: if tokens.next() == ";"
            match tokens.next() {
                None => Err(InvalidInstr),
                Some(arg) => Ok(arg),
            }
        }
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
            "msg" => {
                let args = line
                    .strip_prefix("msg")
                    .expect("Msg already part of line")
                    .split(',')
                    .map(str::trim)
                    .map(str::parse)
                    .try_collect()?;
                Msg(args)
            }
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
}

#[test]
fn scan_test() {
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
",
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

    for (prg, exp) in simple_programs.iter().zip(expected) {
        let actual = AssemblerInterpreter::interpret(*prg);
        assert_eq!(actual, *exp);
    }
}
