use std::iter::Peekable;
use std::str::Chars;

type IT<'a> = Peekable<Chars<'a>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    BrOpen,
    BrClose,
    Add,
    Sub,
    Div,
    Mul,
    Num(i64),
}

#[derive(Clone)]
pub struct Tokenizer<'a> {
    it: IT<'a>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(s: &'a str) -> Self {
        Tokenizer {
            it: s.chars().peekable(),
        }
    }
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

pub fn num_token(it: &mut IT, c: char) -> i64 {
    let mut res = c as i64 - 48;
    while let Some(c) = it.peek() {
        if is_digit(*c) {
            res = res * 10 + *c as i64 - 48;
        } else {
            return res;
        }
        it.next();
    }
    res
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.it.next() {
            match c {
                ' ' | '\n' | '\t' => {}
                '-' => return Some(Ok(Token::Sub)),
                '+' => return Some(Ok(Token::Add)),
                '*' => return Some(Ok(Token::Mul)),
                '/' => return Some(Ok(Token::Div)),
                '(' => return Some(Ok(Token::BrOpen)),
                ')' => return Some(Ok(Token::BrClose)),
                v if is_digit(v) => return Some(Ok(Token::Num(num_token(&mut self.it, c)))),
                c => return Some(Err(format!("Unexpected '{}'", c))),
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tokenizer() {
        let mut tk = Tokenizer::new("34+45     + (34)");
        assert_eq!(tk.next(), Some(Ok(Token::Num(34))));
        assert_eq!(tk.next(), Some(Ok(Token::Add)));
        assert_eq!(tk.next(), Some(Ok(Token::Num(45))));
        assert_eq!(tk.next(), Some(Ok(Token::Add)));
        assert_eq!(tk.next(), Some(Ok(Token::BrOpen)));
        assert_eq!(tk.next(), Some(Ok(Token::Num(34))));
        assert_eq!(tk.next(), Some(Ok(Token::BrClose)));
    }
}
