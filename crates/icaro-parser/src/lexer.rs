//! Hand written lexer with mutable state. Here I keep
//! all of the entry unaltered just using references to parts
//! of the text inside each token.

use std::{iter::Peekable, str::Chars};

use icaro_tree::range::{Position, Range};

use crate::token::Token;
pub struct Lexer<'content> {
    input: &'content str,
    peekable: Peekable<Chars<'content>>,
    current_pos: Position,
    start_pos: Position,
}

fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\r' | '\t')
}

fn is_valid_identifier(c: char) -> bool {
    !matches!(c, '(' | ')' | ' ' | '\r' | '\t')
}

impl<'content> Lexer<'content> {
    pub fn new(input: &'content str) -> Lexer<'content> {
        let peekable = input.chars().peekable();
        Lexer {
            input,
            peekable,
            current_pos: Position::default(),
            start_pos: Position::default(),
        }
    }

    fn range(&mut self) -> Range {
        Range::new(self.start_pos.clone(), self.current_pos.clone())
    }

    fn next_char(&mut self) -> Option<char> {
        let current = self.peekable.next();

        if let Some(chr) = current {
            self.current_pos.advance(chr);
        }

        current
    }

    fn accumulate_while(&mut self, condition: fn(char) -> bool) -> &str {
        let start = self.current_pos.clone();
        while let Some(&x) = self.peekable.peek() {
            if !condition(x) {
                break;
            }
            self.next_char();
        }
        &self.input[start.index..self.current_pos.index]
    }

    fn get_single_token(&mut self, token: Token) -> (Token, Range) {
        self.next_char();
        (token, self.range())
    }

    pub fn tokenize(&mut self) -> (Token, Range) {
        self.start_pos = self.current_pos.clone();
        match self.peekable.peek() {
            None => (Token::EOF, self.range()),
            Some(c) => match c {
                '0'..='9' => {
                    let num = self.accumulate_while(is_valid_identifier);
                    (Token::Int(num.parse().unwrap()), self.range())
                }
                c if is_whitespace(*c) => {
                    self.accumulate_while(is_whitespace);
                    self.tokenize()
                }
                c if is_valid_identifier(*c) => {
                    let str = self.accumulate_while(is_valid_identifier);
                    (Token::Id(str.to_string()), self.range())
                }
                '(' => self.get_single_token(Token::LPar),
                ')' => self.get_single_token(Token::RPar),
                _ => {
                    self.next_char();
                    (Token::Err, self.range())
                }
            },
        }
    }
}
