use std::{mem, ops::Range};

use crate::{
    repr::{expr::ExprRepr, CanParse},
    scanner::Scanner,
    token::Token,
    token_type::TokenType,
};
/// Generates 1 `TokenType` if inputted, but if multiple are it will form a match pattern
macro_rules! ty {
    ($variant: ident) => {
        TokenType::$variant
    };
    ($v1: ident, $($variant: ident),*) => {
        ty!($v1) $(| ty!($variant))*
    };
}

mod prec;
use prec::Precedence;

pub struct Parser<'src, 'repr, R>
where
    R: CanParse + ?Sized,
    Self: Sized,
{
    file_name: String,
    source: &'src String,
    scanner: Scanner<'src>,
    main_repr: &'repr mut R,
    current: Token,
    prev: Token,
    is_main_file: bool,
}
impl<'src, 'repr, R> Parser<'src, 'repr, R>
where
    R: CanParse + ?Sized,
{
    pub fn new(name: String, source: &'src String, repr: &'repr mut R, is_main_file: bool) -> Self {
        Self {
            file_name: name.clone(),
            source,
            scanner: Scanner::new(name, source),
            main_repr: repr,
            current: Token::eof(0, 0..0),
            prev: Token::eof(0, 0..0),
            is_main_file,
        }
    }

    fn next(&mut self) -> Result<&Token, String> {
        let new = match self.scanner.scan() {
            Ok(t) => t,
            Err(e) => {
                eprintln!(
                    "[Scanner Error]: \n\t{} \n\tat [{}:{}:{}]",
                    e.0, self.file_name, e.1, e.2
                );
                self.scanner.scan().map_err(|err| {
                    format!(
                        "[Scanner Error]: \n\t{}\n\tat [{}:{}:{}]",
                        err.0, self.file_name, err.1, err.2
                    )
                })?
            }
        };
        let _ = mem::replace(&mut self.prev, mem::replace(&mut self.current, new));
        Ok(&self.current)
    }

    /// gets the INFIX precedence of the current Token
    fn get_prec(&self) -> Precedence {
        match self.current.kind {
            ty!(LeftParen) => Precedence::Primary,

            ty!(Dot, ColonColon) => Precedence::Get,

            ty!(LeftBracket) => Precedence::Index,

            ty!(Power) => Precedence::Exponent,

            ty!(Star, Slash, Percent) => Precedence::Factor,

            ty!(Plus, Minus) => Precedence::Term,

            ty!(ExclusiveRange, InclusiveRange, RangeFrom, RangeTo) => Precedence::Range,

            ty!(Less, Greater, LessEqual, GreaterEqual) => Precedence::Cmp,

            ty!(EqualEqual, BangEqual, Is) => Precedence::Eq,

            ty!(BitAnd, BitOr, BitXor) => Precedence::Bitwise,

            ty!(And, Or) => Precedence::Logical,

            _ => Precedence::None,
        }
    }
    fn parse_prec(&mut self, precedence: Precedence) -> Result<ExprRepr, String> {
        let mut expr = self.parse_prefix()?;

        while self.get_prec() < precedence {
            expr = self.parse_infix(expr)?;
        }

        Ok(expr)
    }
    fn parse_prefix(&mut self) -> Result<ExprRepr, String> {
        todo!()
    }
    fn parse_infix(&mut self, expr: ExprRepr) -> Result<ExprRepr, String> {
        todo!()
    }

    /// Returns true if an error occurred, and false if not.
    pub fn parse(&mut self) -> bool {
        #[allow(unused_mut)]
        let mut had_err = false;

        while {
            let _ = self.next();
            self.current.kind != TokenType::EOF
        } {
            todo!()
        }

        had_err
    }

    fn synchronize(&mut self) {}
}
