mod token;

use pest::iterators::*;
use pest_derive::*;
use std::str::FromStr;
use token::*;
// use token::{Tokenizer,Token};

#[derive(Parser)]
#[grammar = "sums.pest"]
pub struct SumsParser;

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Brackets(Box<Expr>),
    Op(Oper, Box<Expr>, Box<Expr>),
    Num(i64),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Oper {
    Add,
    Sub,
    Div,
    Mul,
}

pub fn op(o: Oper, a: Expr, b: Expr) -> Expr {
    Expr::Op(o, Box::new(a), Box::new(b))
}

impl FromStr for Expr {
    type Err = String;

    fn from_str(s: &str) -> Result<Expr, String> {
        let mut sp = SumsParser::parse(Rule::sub, s).map_err(|e| e.to_string())?;
        Ok(pair_to_expr(sp.next().unwrap()))
    }
}

// impl FromStr for Expr {
//     type Err = String;

//     fn from_str(s: &str) -> Result<Expr, String> {
//         let t = Tokenizer::new(s);
//         let (_, e) = sub(&t)?;
//         Ok(e)
//     }
// }

//Panics if r is not an operator
pub fn rule_to_op(r: &Rule) -> Oper {
    match r {
        Rule::sub => Oper::Sub,
        Rule::div => Oper::Div,
        Rule::add => Oper::Add,
        Rule::mul => Oper::Mul,
        _ => unreachable! {},
    }
}

fn pair_to_expr(p: Pair<Rule>) -> Expr {
    let rule = p.as_rule();
    if Rule::number == rule {
        return Expr::Num(p.as_str().parse::<i64>().unwrap());
    }
    //iterate over the child rules.
    let mut inner = p.into_inner();
    let left = pair_to_expr(inner.next().unwrap());
    let right = inner.next().map(pair_to_expr);

    match rule {
        Rule::item => return left,
        Rule::brackets => return Expr::Brackets(Box::new(left)),
        Rule::div | Rule::mul | Rule::sub | Rule::add => match right {
            Some(r) => op(rule_to_op(&rule), left, r),
            None => left,
        },
        _ => unreachable! {},
    }
}

pub type ParseRes<'a, T> = Result<(Tokenizer<'a>, T), String>;

pub fn token_bool<'a, F: Fn(&Token) -> bool>(t: &Tokenizer<'a>, f: F) -> ParseRes<'a, Token> {
    let mut it = t.clone();
    match it.next() {
        Some(Ok(v)) if f(&v) => Ok((it, v)),
        _ => Err("Token bool test failed".to_string()),
    }
}

pub fn brackets<'a>(t: &Tokenizer<'a>) -> ParseRes<'a, Expr> {
    let (it, _) = token_bool(t, |t| *t == Token::BrOpen)?;
    let (it, res) = sub(&it)?;
    let (it, _) = token_bool(&it, |t| *t == Token::BrClose)?;

    Ok((it, Expr::Brackets(Box::new(res))))
}

pub fn item<'a>(t: &Tokenizer<'a>) -> ParseRes<'a, Expr> {
    if let Ok(v) = brackets(t) {
        return Ok(v);
    }
    let mut it = t.clone();
    match it.next() {
        Some(Ok(Token::Num(n))) => Ok((it, Expr::Num(n))),
        _ => Err("No number or brackets found".to_string()),
    }
}

// ordered by B.I.M.D.A.S. (Brackets,Indices,Multiplication,Addition,Subtraction)
pub fn div<'a>(t: &Tokenizer<'a>) -> ParseRes<'a, Expr> {
    // Left hand side
    let (it, left) = item(t)?;
    // Handle Divide Symbol, if no divide just return the left
    if let Ok((divit, _)) = token_bool(&it, |v| *v == Token::Div) {
        let (rit, right) = div(&divit)?;
        Ok((rit, op(Oper::Div, left, right)))
    } else {
        Ok((it, left))
    }
}

pub fn mul<'a>(t: &Tokenizer<'a>) -> ParseRes<'a, Expr> {
    // Left hand side
    let (it, left) = div(t)?;
    // Handle Mul Symbol, if no multiply just return the left
    if let Ok((divit, _)) = token_bool(&it, |v| *v == Token::Mul) {
        let (rit, right) = mul(&divit)?;
        Ok((rit, op(Oper::Mul, left, right)))
    } else {
        Ok((it, left))
    }
}

pub fn add<'a>(t: &Tokenizer<'a>) -> ParseRes<'a, Expr> {
    // Left hand side
    let (it, left) = mul(t)?;
    // Handle Add Symbol, if no addition just return the left
    if let Ok((divit, _)) = token_bool(&it, |v| *v == Token::Add) {
        let (rit, right) = add(&divit)?;
        Ok((rit, op(Oper::Add, left, right)))
    } else {
        Ok((it, left))
    }
}

pub fn sub<'a>(t: &Tokenizer<'a>) -> ParseRes<'a, Expr> {
    // Left hand side
    let (it, left) = add(t)?;
    // Handle Add Symbol, if no addition just return the left
    if let Ok((divit, _)) = token_bool(&it, |v| *v == Token::Sub) {
        let (rit, right) = sub(&divit)?;
        Ok((rit, op(Oper::Sub, left, right)))
    } else {
        Ok((it, left))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pest::*;
    #[test]
    // fn test_strs() {
    //     let e: Expr = "3 + 5 *(7-3)".parse().unwrap();
    //     assert_eq!(
    //         e,
    //         op(
    //             Oper::Add,
    //             Expr::Num(3),
    //             op(
    //                 Oper::Mul,
    //                 Expr::Num(5),
    //                 Expr::Brackets(Box::new(op(Oper::Sub, Expr::Num(7), Expr::Num(3))))
    //             )
    //         )
    //     )
    // }
    #[test]
    pub fn test_pest_returns_something() {
        let mut sp = SumsParser::parse(Rule::sub, "23+4").unwrap();
        assert_eq!(sp.next().unwrap().as_rule(), Rule::sub);
    }
}

fn main() {
    println!("Hello, world!");
}
