//NOTE: https://www.codewars.com/kata/52a78825cdfc2cfc87000005/train/rust

use itertools::{Itertools, PeekingNext};
use std::borrow::Cow;
use std::iter;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Clone, Copy)]
struct TokenInfo {
    index: usize,
    len: usize,
    t_type: Token,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    Space,
    Op(char),
    LeftParen,
    RightParen,
    Num(f64),
}

/// Syntax
///
/// expression     → term
///
/// term           → factor ( ( "-" | "+" ) factor )*
///
/// factor         → unary ( ( "/" | "*" ) unary )*
///
/// unary          → "-" unary | primary
///
/// primary        → "(" term ")" | number
// TODO: create EType and Expr (store tokens there)
#[derive(Debug, PartialEq, Clone)]
enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Grouping(Box<Expr>),
    Num(f64),
}

impl Expr {
    fn from(tokens: impl Iterator<Item = TokenInfo>) -> Result<Expr, Cow<'static, str>> {
        let mut peekable = tokens.peekable();
        let expr = term(&mut peekable);
        return match peekable.next() {
            Some(token) => Err(format!("Invalid token! index:{0}", token.index).into()),
            None => expr,
        };

        // TODO: replace with impl PeekingNext<Item = Token>
        fn term(
            tokens: &mut Peekable<impl Iterator<Item = TokenInfo>>,
        ) -> Result<Expr, Cow<'static, str>> {
            next_if_space(tokens);
            let mut left_expr = factor(tokens)?;
            while let Some(token) =
                tokens.peeking_next(|token| matches!(token.t_type, Token::Op('+' | '-')))
            {
                next_if_space(tokens);
                let left = Box::new(left_expr);
                let right = Box::new(factor(tokens)?);
                left_expr = Expr::Binary(left, token.t_type, right);
            }
            Ok(left_expr)
        }

        fn factor(
            tokens: &mut Peekable<impl Iterator<Item = TokenInfo>>,
        ) -> Result<Expr, Cow<'static, str>> {
            next_if_space(tokens);
            let mut left_expr = unary(tokens)?;
            while let Some(token) =
                tokens.peeking_next(|token| matches!(token.t_type, Token::Op('*' | '/')))
            {
                next_if_space(tokens);
                let left = Box::new(left_expr);
                let right = Box::new(unary(tokens)?);
                left_expr = Expr::Binary(left, token.t_type, right);
            }
            Ok(left_expr)
        }

        fn unary(
            tokens: &mut Peekable<impl Iterator<Item = TokenInfo>>,
        ) -> Result<Expr, Cow<'static, str>> {
            let expr = match tokens.peek() {
                None => Err("Invalid ending!".into()),
                Some(&token) => match token.t_type {
                    Token::Op('-') => {
                        tokens.next();
                        let expr = Box::new(unary(tokens)?);
                        Ok(Expr::Unary(token.t_type, expr))
                    }
                    _ => primary(tokens),
                },
            };
            next_if_space(tokens);
            expr
        }

        fn primary(
            tokens: &mut Peekable<impl Iterator<Item = TokenInfo>>,
        ) -> Result<Expr, Cow<'static, str>> {
            let token = tokens.next().ok_or("Invalid ending!")?;
            match token.t_type {
                Token::Num(num) => Ok(Expr::Num(num)),
                Token::LeftParen => {
                    let expr = term(tokens)?;
                    match tokens.next() {
                        Some(token) => match token.t_type {
                            Token::RightParen => Ok(Expr::Grouping(Box::new(expr))),
                            _ => Err(format!("Invalid paren! index:{0}", token.index).into()),
                        },
                        None => Err("Invalid paren!".into()),
                    }
                }
                _ => Err(format!("Invalid token! index:{0}", token.index).into()),
            }
        }

        fn next_if_space(tokens: &mut Peekable<impl Iterator<Item = TokenInfo>>) {
            tokens.peeking_next(|token| matches!(token.t_type, Token::Space));
        }
    }

    fn eval(&self) -> f64 {
        match self {
            Expr::Binary(left, Token::Op(ch), right) => match ch {
                '+' => left.eval() + right.eval(),
                '-' => left.eval() - right.eval(),
                '/' => left.eval() / right.eval(),
                '*' => left.eval() * right.eval(),
                op => panic!("Invalid operation! op:{op:?}"),
            },
            Expr::Unary(Token::Op('-'), expr) => -expr.eval(),
            Expr::Grouping(expr) => expr.eval(),
            Expr::Num(num) => *num,
            _ => panic!("Invalid expression! expr:{self:?}"),
        }
    }
}

pub fn calc(input_expr: &str) -> f64 {
    let tokens = scan(input_expr);
    let expr = Expr::from(tokens).unwrap();
    expr.eval()
}

#[rustfmt::skip]
pub fn scan(str: &str) -> impl Iterator<Item =TokenInfo> + '_{
    str.char_indices().peekable().batching(|iter| {
        match iter.next() {
            None => None,
            Some((index, ch)) => {
                let token = match ch {
                    '0'..='9' => {
                        let num_str: String = iter::once(ch)
                            .chain(iter
                                .peeking_take_while(|(_, ch)| ch.is_numeric() || *ch == '.')
                                .map(|(_, ch)| ch))
                            .collect();
                        let num = num_str.parse::<f64>().unwrap_or_else(|_| panic!("Invalid token! index:{index}"));
                        TokenInfo { index, len: num_str.len(), t_type: Token::Num(num) }
                    },
                    '(' => TokenInfo { index, len: 1, t_type: Token::LeftParen },
                    ')' => TokenInfo { index, len: 1, t_type: Token::RightParen },
                    '+' | '-' | '/' | '*' => TokenInfo { index, len: 1, t_type: Token::Op(ch) },
                    ' ' => {
                        let last_index = iter
                            .peeking_take_while(|(_, ch)| ch.is_whitespace())
                            .map(|(index, _)| index)
                            .last()
                            .unwrap_or(index);
                        TokenInfo { index, len: 1 + last_index - index, t_type: Token::Space }
                    }
                    _ => panic!("Invalid token! index:{index}"),
                };
                Some(token)
            }
        }
    })
}

#[test]
fn scan_test() {
    let test = "1 + -2.24   (   -12323";
    let tokens = scan(test).map(|token| token.t_type);
    let expected_tokens = [
        Token::Num(1.0),
        Token::Space,
        Token::Op('+'),
        Token::Space,
        Token::Op('-'),
        Token::Num(2.24),
        Token::Space,
        Token::LeftParen,
        Token::Space,
        Token::Op('-'),
        Token::Num(12323.0),
    ];
    itertools::assert_equal(tokens, expected_tokens);

    let test = "1.0  ";
    let tokens = scan(test);
    let expected_tokens = [
        TokenInfo {
            index: 0,
            len: 3,
            t_type: Token::Num(1.0),
        },
        TokenInfo {
            index: 3,
            len: 2,
            t_type: Token::Space,
        },
    ];
    itertools::assert_equal(tokens, expected_tokens);
}

macro_rules! assert_expr_eq {
    ($expr: expr, $expect: expr) => {
        assert!(
            calc($expr).eq(&$expect),
            "\nexpected expression \"{}\" to equal \"{:?}\", but got \"{:?}\"",
            $expr,
            $expect,
            calc($expr),
        );
    };
}

#[test]
fn fail_test() {
    assert_expr_eq!("1 - -1", 2.0);
}

#[test]
fn single_values() {
    assert_expr_eq!("0", 0.0);
    assert_expr_eq!("1", 1.0);
    assert_expr_eq!("42", 42.0);
    assert_expr_eq!("350", 350.0);
}

#[test]
fn basic_operations() {
    assert_expr_eq!("1 + 1", 2.0);
    assert_expr_eq!("1 - 1", 0.0);
    assert_expr_eq!("1 * 1", 1.0);
    assert_expr_eq!("1 / 1", 1.0);
    assert_expr_eq!("12 * 123", 1476.0);
}

#[test]
fn whitespace_between_operators_and_operands() {
    assert_expr_eq!("1-1", 0.0);
    assert_expr_eq!("1 -1", 0.0);
    assert_expr_eq!("1- 1", 0.0);
    assert_expr_eq!("1* 1", 1.0);
}

#[test]
fn unary_minuses() {
    assert_expr_eq!("1- -1", 2.0);
    assert_expr_eq!("1--1", 2.0);
    assert_expr_eq!("1 - -1", 2.0);
    assert_expr_eq!("-42", -42.0);
}

#[test]
fn parentheses() {
    assert_expr_eq!("(1)", 1.0);
    assert_expr_eq!("((1))", 1.0);
    assert_expr_eq!("((80 - (19)))", 61.0);
}

#[test]
fn multiple_operators() {
    assert_expr_eq!("12* 123/(-5 + 2)", -492.0);
    assert_expr_eq!("1 - -(-(-(-4)))", -3.0);
    assert_expr_eq!("2 /2+3 * 4.75- -6", 21.25);
    assert_expr_eq!("2 / (2 + 3) * 4.33 - -6", 7.732);
    assert_expr_eq!("(1 - 2) + -(   -(-(-4)))", 3.0);
    assert_expr_eq!("((2.33 / (2.9+3.5)*4) - -6)", 7.45625);
    assert_expr_eq!("((-6 / 0.75 * -0.375) + (4.5 * 72 / 36))", 12.0);
}
