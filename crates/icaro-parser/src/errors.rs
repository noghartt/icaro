use icaro_tree::range::Range;

use crate::token::Token;

pub enum SyntaticDiagnostic {
    UnclosedParenthesis {
        start: Range,
    },
    UnexpectedToken {
        place: Range,
        token: Token,
        expected: Vec<Token>,
    },
}
