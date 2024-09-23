use std::mem::replace;

use crate::{
    repr::{CanParse, ModuleRepr},
    scanner::{ScanError, Scanner},
    token::Token,
};

pub struct Parser<'src, 'repr, R: CanParse> {
    source: &'src String,
    scanner: Scanner<'src>,
    repr: &'repr mut R,
    current: Token,
    prev: Token,
    is_main_file: bool,
}
impl<'src, 'repr, R: CanParse> Parser<'src, 'repr, R> {
    pub fn new(source: &'src String, repr: &'repr mut R, is_main_file: bool) -> Self {
        Self {
            source,
            scanner: Scanner::new(source),
            repr,
            current: Token::eof(0, 0..0),
            prev: Token::eof(0, 0..0),
            is_main_file,
        }
    }

    fn next(&mut self) -> Result<&Token, ScanError> {
        let new = self.scanner.scan()?;
        let _ = replace(&mut self.prev, replace(&mut self.current, new));
        Ok(&self.current)
    }
    pub fn parse(&mut self) -> bool {
        true
    }
}
