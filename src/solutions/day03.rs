use std::iter::{self, from_fn};

use crate::{create_solver, Part};

create_solver!(part_one, part_two);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Token {
    Number(i64),
    Do,
    Dont,
    Mul,
    OpenParen,
    CloseParen,
    Comma,
    Ignore,
}

fn tokenize(input: Vec<u8>) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut iter = input.iter().map(|&i| i as char).peekable();

    while let Some(c) = iter.next() {
        match c {
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            ',' => tokens.push(Token::Comma),
            '0'..='9' => {
                let n = iter::once(c)
                    .chain(from_fn(|| iter.by_ref().next_if(|s| s.is_ascii_digit())))
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap();
                if n >= 0 && n < 1000 {
                    tokens.push(Token::Number(n));
                }
            }
            'm' => {
                let m = iter
                    .by_ref()
                    .next_if_eq(&'u')
                    .and(iter.by_ref().next_if_eq(&'l'));
                if m.is_some() {
                    tokens.push(Token::Mul);
                }
            }
            'd' => {
                let is_do = iter.by_ref().next_if_eq(&'o');

                let is_dont = is_do
                    .and(iter.by_ref().next_if_eq(&'n'))
                    .and(iter.by_ref().next_if_eq(&'\''))
                    .and(iter.by_ref().next_if_eq(&'t'));

                if is_dont.is_some() {
                    tokens.push(Token::Dont);
                } else if is_do.is_some() {
                    tokens.push(Token::Do);
                }
            }
            _ => tokens.push(Token::Ignore),
        }
    }
    tokens
}

fn part_one(input: Vec<u8>) -> i64 {
    let tokens = tokenize(input);
    let mut result = 0;

    let mut iter = tokens.iter().peekable();
    while let Some(&token) = iter.next() {
        match token {
            Token::Mul => {
                if None == iter.next_if(|&&t| t == Token::OpenParen) {
                    continue;
                }
                let a = iter.next_if(|&&t| matches!(t, Token::Number(_)));
                if a.is_none() {
                    continue;
                }

                if None == iter.next_if(|&&t| t == Token::Comma) {
                    continue;
                }
                let b = iter.next_if(|&&t| matches!(t, Token::Number(_)));
                if b.is_none() {
                    continue;
                }

                if None == iter.next_if(|&&t| t == Token::CloseParen) {
                    continue;
                }

                if let (Some(Token::Number(a)), Some(Token::Number(b))) = (a, b) {
                    result += a * b;
                }
            }
            _ => {}
        }
    }

    result
}

fn part_two(input: Vec<u8>) -> i64 {
    let tokens = tokenize(input);
    let mut result = 0;
    let mut include = true;

    let mut iter = tokens.iter().peekable();
    while let Some(&token) = iter.next() {
        match token {
            Token::Do => {
                let d = iter
                    .next_if_eq(&&Token::OpenParen)
                    .and(iter.by_ref().next_if_eq(&&Token::CloseParen));
                if d.is_some() {
                    include = true
                }
            }
            Token::Dont => {
                let d = iter
                    .next_if_eq(&&Token::OpenParen)
                    .and(iter.by_ref().next_if_eq(&&Token::CloseParen));
                if d.is_some() {
                    include = false
                }
            }
            Token::Mul => {
                if None == iter.next_if(|&&t| t == Token::OpenParen) {
                    continue;
                }
                let a = iter.next_if(|&&t| matches!(t, Token::Number(_)));
                if a.is_none() {
                    continue;
                }

                if None == iter.next_if(|&&t| t == Token::Comma) {
                    continue;
                }
                let b = iter.next_if(|&&t| matches!(t, Token::Number(_)));
                if b.is_none() {
                    continue;
                }

                if None == iter.next_if(|&&t| t == Token::CloseParen) {
                    continue;
                }

                if let (Some(Token::Number(a)), Some(Token::Number(b))) = (a, b) {
                    if include {
                        result += a * b;
                    }
                }
            }
            _ => {}
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#
            .as_bytes()
            .to_vec();
        assert_eq!(part_one(input), 161);
    }

    #[test]
    fn test_part_two() {
        let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#
            .as_bytes()
            .to_vec();
        assert_eq!(part_two(input), 48);
    }
}
