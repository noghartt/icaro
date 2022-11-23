use errors::SyntaticDiagnostic;
use icaro_tree::{range::Range, Expr, Ident};
use lexer::Lexer;

use token::Token;

pub mod errors;
pub mod lexer;
pub mod token;

pub struct Parser<'a> {
    lexer: Lexer<'a>,

    actual: (Token, Range),
    next: (Token, Range),
    count: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        let mut lexer = Lexer::new(input);
        let actual = lexer.tokenize();
        let next = lexer.tokenize();
        Parser {
            lexer,
            actual,
            next,
            count: 0,
        }
    }

    fn advance(&mut self) -> (Token, Range) {
        let older = self.actual.clone();
        self.actual = self.next.clone();
        self.next = self.lexer.tokenize();
        self.count += 1;
        older
    }

    pub fn eat<T>(&mut self, condition: fn(&Token) -> Option<T>) -> Result<T, String> {
        match condition(&self.actual.0) {
            Some(res) => Ok(res),
            None => Err("Does not matches".to_string()),
        }
    }

    pub fn try_single<T, E>(
        &mut self,
        runner: fn(&mut Self) -> Result<T, E>,
    ) -> Result<Option<T>, E> {
        let count = self.count;
        match runner(self) {
            Err(_) if self.count == count => Ok(None),
            Err(err) => Err(err),
            Ok(res) => Ok(Some(res)),
        }
    }

    pub fn eat_exact(&mut self, condition: Token) -> Result<(Token, Range), SyntaticDiagnostic> {
        if condition == self.actual.0 {
            let actual = self.advance();
            Ok(actual)
        } else {
            Err(SyntaticDiagnostic::UnexpectedToken {
                place: self.actual.1.clone(),
                token: self.actual.0.clone(),
                expected: vec![],
            })
        }
    }

    pub fn parse_elem(&mut self) -> Result<Expr, SyntaticDiagnostic> {
        let (token, range) = &self.actual;
        match token {
            Token::Int(num) => Ok(Expr::Int(range.clone(), *num)),
            Token::Str(str) => Ok(Expr::Str(range.clone(), str.to_string())),
            Token::Id(str) => Ok(Expr::Atom(Ident::from_string(
                str.to_string(),
                range.clone(),
            ))),
            Token::LPar => self.parse_list(),
            res => Err(SyntaticDiagnostic::UnexpectedToken {
                place: range.clone(),
                token: res.clone(),
                expected: vec![],
            }),
        }
    }

    pub fn parse_list(&mut self) -> Result<Expr, SyntaticDiagnostic> {
        let (_, start) = self.eat_exact(Token::LPar)?;

        let mut list = Vec::new();

        while let Some(elem) = self.try_single(|this| this.parse_elem())? {
            list.push(elem)
        }

        if self.eat_exact(Token::RPar).is_err() {
            Err(SyntaticDiagnostic::UnclosedParenthesis { start })
        } else {
            Ok(Expr::List(list))
        }
    }
}
