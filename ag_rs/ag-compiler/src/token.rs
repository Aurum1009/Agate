use crate::token_type::TokenType;
use std::ops::Range;

pub struct Token {
    pub(crate) kind: TokenType,
    // Boxing makes it more cache-friendly
    pub(crate) lexeme: Box<String>,
    pub(crate) line: usize,
    pub(crate) columns: Range<usize>,
}
impl Token {
    pub fn new(kind: TokenType, lexeme: String, line: usize, columns: Range<usize>) -> Self {
        Self {
            kind,
            lexeme: Box::new(lexeme),
            line,
            columns,
        }
    }
    pub fn eof(line: usize, columns: Range<usize>) -> Self {
        Self {
            kind: TokenType::EOF,
            lexeme: Box::new(String::new()),
            line,
            columns,
        }
    }
}
