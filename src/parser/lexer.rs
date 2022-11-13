use std::{iter::Peekable, str::Chars};

use super::token::Token;
use super::range::{Range, Position};

/// The lexer state
#[derive(Debug)]
pub struct Lexer<'a> {
    pub input: &'a str,
    pub position: usize,
    pub peekable: &'a mut Peekable<Chars<'a>>
}

fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\r' | '\t')
}

impl<'a> Lexer<'a> {
    pub fn new(
        input: &'a str,
        peekable: &'a mut Peekable<Chars<'a>>
    ) -> Lexer {
        Lexer {
            input,
            position: 0,
            peekable,
        }
    }

    pub fn tokenize(&mut self) -> (Token, Range) {
        let start = self.position;

        match self.peekable.peek() {
            None => (Token::EOF, self.mk_range(start)),
            Some(c) => match c {
                c if is_whitespace(*c) => {
                    self.accumulate_while(&is_whitespace);
                    self.tokenize()
                }
                // TODO: Add other types of numbers on it too
                c if c.is_ascii_digit() => {
                    let num = self.accumulate_while(&|x| x.is_ascii_digit() || x == '_');
                    (Token::LInt(num.replace('_', "").parse().unwrap()), self.mk_range(start))
                }
                '(' => self.to_single_token(Token::LPar, start),
                ')' => self.to_single_token(Token::RPar, start),
                '+' => self.to_single_token(Token::OpPlus, start),
                _ => unreachable!(),
            }
        }
    }

    fn to_single_token(&mut self, token: Token, start: usize) -> (Token, Range) {
        self.next_char();
        (token, self.mk_range(start))
    }

    fn next_char(&mut self) -> Option<char> {
        match self.peekable.next() {
            Some(c) if !self.input.is_empty() => {
                self.input = &self.input[c.len_utf8()..];
                self.position += c.len_utf8();
                Some(c)
            }
            _ => None
        }
    }

    fn mk_range(&self, start: usize) -> Range {
        Range::new(
            Position { index: start as u32 },
            Position { index: self.position as u32 }
        )
    }

    fn accumulate_while(&mut self, condition: &dyn Fn(char) -> bool) -> &str {
        let mut size = 0;

        while let Some(&x) = self.peekable.peek() {
            if !condition(x) {
                break;
            }

            size += x.len_utf8();

            self.peekable.next();
        }

        self.position += size;

        let str = &self.input[..size];

        self.input = &self.input[size..];

        str
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use super::Token;

    #[test]
    fn assert_next_char() {
        let input = "(+ 1 2)";
        let mut input_peek = input.chars().peekable();

        let mut lexer = Lexer::new(input, &mut input_peek);

        let c = lexer.next_char();
        assert_eq!(c, Some('('));

        let c = lexer.next_char();
        assert_eq!(c, Some('+'));
    }

    #[test]
    fn should_tokenize() {
        let input = "(+ 1 2)";
        let mut input_peek = input.chars().peekable();

        let mut lexer = Lexer::new(input, &mut input_peek);

        let (token, _range) = lexer.tokenize();
        assert_eq!(token, Token::LPar);

        let (token, _range) = lexer.tokenize();
        assert_eq!(token, Token::OpPlus);

        let (token, _range) = lexer.tokenize();
        assert_eq!(token, Token::LInt(1));

        let (token, _range) = lexer.tokenize();
        assert_eq!(token, Token::LInt(2));

        let (token, _range) = lexer.tokenize();
        assert_eq!(token, Token::RPar);
    }
}
