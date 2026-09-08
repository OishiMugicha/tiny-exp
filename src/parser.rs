use std::{iter::Peekable, slice::Iter};

use crate::{ast::*, error::Error, lexer::Token};

/// 優先順位に従って式を解析し、入力全体を消費します。
pub fn parse(tokens: &[Token]) -> Result<Exp, Error> {
    if tokens.is_empty() {
        return Err(Error::Parse("Empty input".to_string()));
    }
    let mut iter = tokens.iter().peekable();
    let expr = parse_expr(&mut iter)?;
    if iter.next().is_some() {
        Err(Error::Parse("Unexpected token".to_string()))
    } else {
        Ok(expr)
    }
}

type Tokens<'a> = Peekable<Iter<'a, Token>>;

fn eat(iter: &mut Tokens<'_>, expected: Token) -> Result<(), Error> {
    match iter.next() {
        Some(token) if *token == expected => Ok(()),
        Some(token) => Err(Error::Parse(format!(
            "Expected {:?}, but found {:?}",
            expected, token
        ))),
        None => Err(Error::Parse(format!(
            "Expected {:?}, but found end of input",
            expected
        ))),
    }
}

fn parse_expr(iter: &mut Tokens<'_>) -> Result<Exp, Error> {
    match iter.peek() {
        Some(Token::If) => {
            eat(iter, Token::If)?;
            let cond = parse_expr(iter)?;
            eat(iter, Token::Then)?;
            let then_branch = parse_expr(iter)?;
            eat(iter, Token::Else)?;
            let else_branch = parse_expr(iter)?;
            Ok(Exp::If(
                Box::new(cond),
                Box::new(then_branch),
                Box::new(else_branch),
            ))
        }
        Some(_) => {
            let formula = parse_formula(iter)?;
            Ok(formula)
        }
        None => Err(Error::Parse("missing token".to_string())),
    }
}

fn parse_formula(iter: &mut Tokens<'_>) -> Result<Exp, Error> {
    let mut left = parse_additive(iter)?;
    if let Some(Token::Less) = iter.peek() {
        eat(iter, Token::Less)?;
        let right = parse_additive(iter)?;
        left = Exp::Binary(Box::new(left), Prim::Lt, Box::new(right));
    }
    Ok(left)
}

fn parse_additive(iter: &mut Tokens<'_>) -> Result<Exp, Error> {
    let mut left = parse_multiplicative(iter)?;
    loop {
        match iter.peek() {
            Some(Token::Plus) => {
                iter.next(); // consume '+'
                let right = parse_multiplicative(iter)?;
                left = Exp::Binary(Box::new(left), Prim::Add, Box::new(right));
            }
            Some(Token::Minus) => {
                iter.next(); // consume '-'
                let right = parse_multiplicative(iter)?;
                left = Exp::Binary(Box::new(left), Prim::Sub, Box::new(right));
            }
            _ => break,
        }
    }
    Ok(left)
}

fn parse_multiplicative(iter: &mut Tokens<'_>) -> Result<Exp, Error> {
    let mut left = parse_unary(iter)?;
    while let Some(Token::Star) = iter.peek() {
        iter.next(); // consume '*'
        let right = parse_unary(iter)?;
        left = Exp::Binary(Box::new(left), Prim::Mul, Box::new(right));
    }
    Ok(left)
}

fn parse_unary(iter: &mut Tokens<'_>) -> Result<Exp, Error> {
    if let Some(Token::Minus) = iter.peek() {
        iter.next();
        match iter.next() {
            Some(Token::Integer(n)) => parse_integer(n, true),
            _ => Err(Error::Parse("Expected integer after '-'".to_string())),
        }
    } else {
        parse_primary(iter)
    }
}

fn parse_integer(digits: &str, negative: bool) -> Result<Exp, Error> {
    if digits.is_empty() || !digits.bytes().all(|c| c.is_ascii_digit()) {
        return Err(Error::Parse(format!("Invalid integer: {digits}")));
    }
    let signed = if negative {
        format!("-{digits}")
    } else {
        digits.to_owned()
    };
    let value = signed.parse::<i64>().map_err(|_| Error::Overflow)?;
    Ok(Exp::Literal(Value::Int(value)))
}

fn parse_primary(iter: &mut Tokens<'_>) -> Result<Exp, Error> {
    match iter.next() {
        Some(Token::Integer(n)) => parse_integer(n, false),
        Some(Token::Bool(b)) => Ok(Exp::Literal(Value::Bool(*b))),
        Some(Token::LeftParen) => {
            let expr = parse_expr(iter)?;
            match iter.next() {
                Some(Token::RightParen) => Ok(expr),
                _ => Err(Error::Parse("Expected ')'".to_string())),
            }
        }
        _ => Err(Error::Parse("Unexpected token".to_string())),
    }
}
