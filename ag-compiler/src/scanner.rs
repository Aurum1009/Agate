use super::{token::*, token_type::*};
use std::ops::Range;

macro_rules! str {
    () => {
        String::new()
    };
    ($slice: literal) => {
        ($slice).to_string()
    };
    (true, $expr: expr) => {
        format!("{}", ($expr))
    };
    (false, $expr: expr) => {
        format!("{:?}", ($expr))
    };
}

macro_rules! ty {
    ($kind: ident) => {
        TokenType::$kind
    };
}

macro_rules! r#match {
    ($scan: ident, $char: expr) => {
        ($scan).r#match(($char))?
    };
}

macro_rules! tok {
    ($scan: expr, $type: ident) => {
        ($scan).tok(ty!($type))
    };
}
macro_rules! ok_tok {
    ($scan: expr, $type: ident) => {
        Ok(tok!($scan, $type))
    };
}

/// The scanner's error type. It consists of a message, the line and the column it occurred on,
/// in the order (message, line, column).
pub(super) type ScanError = (String, usize, usize);
type ScanResult = Result<Token, ScanError>;

/// A scanner that scans through source code and parses it into easy-to-use `Token`s for the compiler.
/// It works with a shared string reference to conserve memory, and uses `Vec<String>` for characters
/// because a `Vec<char>` is inaccurate due to Unicode, and using `String` allows us to fully parse a character
/// thanks to the [unicode_segmentation][https://crates.io/crates/unicode-segmentation] crate.
///
/// It uses a small counter, `current`, to peek past the
pub struct Scanner<'src> {
    source: &'src String,
    chars: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    columns: Range<usize>,
    last: Option<TokenType>,
}
impl<'src> Scanner<'src> {
    pub fn new<'s: 'src>(source: &'s String) -> Self {
        Self {
            source,
            chars: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            columns: 0..0,
            last: None,
        }
    }
    /// Reinitialize the scanner for new source code.
    pub fn re_init<'s: 'src>(&mut self, source: &'s String) {
        self.source = source;
        self.chars = source.chars().collect();
        self.current = 0;
        self.line = 1;
        self.columns = 0..0;
        self.last = None;
    }
    fn is_at_end(&self) -> bool {
        self.current > self.chars.len()
        // ^^^ because we use `[self.current - 1]`
    }
    fn current(&self) -> &char {
        &self.chars[self.current - 1]
    }
    fn advance(&mut self) -> Result<&char, ScanError> {
        self.current += 1;
        self.columns.end += 1;
        if self.is_at_end() {
            Err((
                "Unexpected end of file".to_string(),
                self.line,
                self.columns.end,
            ))
        } else {
            Ok(self.current())
        }
    }
    fn peek(&self) -> &char {
        &self.chars[self.current]
    }
    fn peek_next(&self) -> Option<&char> {
        if self.chars.len() <= self.current + 1 {
            None
        } else {
            Some(&self.chars[self.current + 1])
        }
    }
    fn tok(&self, kind: TokenType) -> Token {
        Token::new(
            kind,
            self.source[self.start..self.current].to_string(),
            self.line,
            self.columns.start..self.columns.end,
        )
    }
    fn r#match(&mut self, kind: char) -> Result<bool, ScanError> {
        Ok(if self.peek() == &kind {
            self.advance()?;
            true
        } else {
            false
        })
    }
    pub fn scan(&mut self) -> ScanResult {
        self.start = self.current;
        let char = match self.advance() {
            Ok(c) => c,
            Err(_) => return Ok(Token::eof(self.line, self.columns.start..self.columns.end)),
        };

        match char {
            // groups
            '(' => ok_tok!(self, LeftParen),
            ')' => ok_tok!(self, RightParen),
            '{' => ok_tok!(self, LeftBrace),
            '}' => ok_tok!(self, RightBrace),
            '[' => ok_tok!(self, LeftBracket),
            ']' => ok_tok!(self, RightBracket),
            // logical
            '=' => {
                if r#match!(self, '=') {
                    ok_tok!(self, EqualEqual)
                } else if r#match!(self, '>') {
                    ok_tok!(self, FatArrow)
                } else {
                    ok_tok!(self, Equal)
                }
            }
            '!' => {
                if r#match!(self, '=') {
                    ok_tok!(self, BangEqual)
                } else {
                    ok_tok!(self, Bang)
                }
            }
            // comparison
            '<' => {
                if r#match!(self, '=') {
                    ok_tok!(self, LessEqual)
                } else if r#match!(self, ':') {
                    ok_tok!(self, Inherits)
                } else {
                    ok_tok!(self, Less)
                }
            }
            '>' => {
                if r#match!(self, '=') {
                    ok_tok!(self, GreaterEqual)
                } else {
                    ok_tok!(self, Greater)
                }
            }
            // arithmetic
            '+' => {
                if r#match!(self, '=') {
                    ok_tok!(self, PlusEqual)
                } else {
                    ok_tok!(self, Plus)
                }
            }
            '-' => {
                if r#match!(self, '=') {
                    ok_tok!(self, MinusEqual)
                } else if r#match!(self, '>') {
                    ok_tok!(self, RetArrow)
                } else {
                    ok_tok!(self, Minus)
                }
            }
            '*' => {
                if r#match!(self, '*') {
                    if r#match!(self, '=') {
                        ok_tok!(self, PowerEqual)
                    } else {
                        ok_tok!(self, Power)
                    }
                } else if r#match!(self, '=') {
                    ok_tok!(self, StarEqual)
                } else {
                    ok_tok!(self, Star)
                }
            }
            '/' => {
                if r#match!(self, '=') {
                    ok_tok!(self, SlashEqual)
                } else {
                    ok_tok!(self, Slash)
                }
            }
            '%' => {
                if r#match!(self, '=') {
                    ok_tok!(self, PercentEqual)
                } else {
                    ok_tok!(self, Percent)
                }
            }
            // bitwise
            '&' => {
                if r#match!(self, '=') {
                    ok_tok!(self, BitAndEqual)
                } else {
                    ok_tok!(self, BitAnd)
                }
            }
            '|' => {
                if r#match!(self, '=') {
                    ok_tok!(self, BitOrEqual)
                } else if r#match!(self, '>') {
                    ok_tok!(self, Pipe)
                } else {
                    ok_tok!(self, BitOr)
                }
            }
            '^' => {
                if r#match!(self, '=') {
                    ok_tok!(self, BitXorEqual)
                } else {
                    ok_tok!(self, BitXor)
                }
            }
            // other
            ';' => ok_tok!(self, Semicolon),
            '\n' => ok_tok!(self, Newline),
            ':' => ok_tok!(self, Colon),
            '.' => {
                if r#match!(self, '.') {
                    if r#match!(self, '=') {
                        ok_tok!(self, InclusiveRange)
                    } else {
                        ok_tok!(self, ExclusiveRange)
                    }
                } else {
                    ok_tok!(self, Dot)
                }
            }
            // literals
            '"' => self.string(),
            '\'' => self.char(),
            c => {
                if c.is_alphabetic() {
                    self.ident()
                } else if c.is_numeric() {
                    self.number()
                } else {
                    Err((
                        format!("Character {c} is not recognized"),
                        self.line,
                        self.columns.end,
                    ))
                }
            }
        }
    }

    fn number(&mut self) -> ScanResult {
        while self.peek().is_numeric() {
            self.advance()?;
        }
        if self.peek() == &'.' && self.peek_next() == Some(&'1') {
            self.advance()?;

            while self.peek().is_numeric() {
                self.advance()?;
            }

            ok_tok!(self, Decimal)
        } else {
            ok_tok!(self, Integer)
        }
    }
    fn ident(&mut self) -> ScanResult {
        while self.peek().is_alphanumeric() {
            self.advance()?;
        }
        let slice = &self.source[self.start..self.current];
        return match slice {
            "and" => ok_tok!(self, And),
            "or" => ok_tok!(self, Or),
            "let" => ok_tok!(self, Let),
            "var" => ok_tok!(self, Var),
            "global" => ok_tok!(self, Global),
            "mut" => ok_tok!(self, Mut),
            "if" => ok_tok!(self, If),
            "unless" => ok_tok!(self, Unless),
            "while" => ok_tok!(self, While),
            "until" => ok_tok!(self, Until),
            "for" => ok_tok!(self, For),
            "in" => ok_tok!(self, In),
            "loop" => ok_tok!(self, Loop),
            "func" => ok_tok!(self, Func),
            "Self" => ok_tok!(self, CapitalSelf),
            "self" => ok_tok!(self, _Self),
            "class" => ok_tok!(self, Class),
            "pub" => ok_tok!(self, Pub),
            "true" => ok_tok!(self, True),
            "false" => ok_tok!(self, False),
            _ => ok_tok!(self, Identifier),
        };
    }
    fn char(&mut self) -> ScanResult {
        // we do not give it a length now, as we need to handle unicode graphemes correctly
        while !r#match!(self, '\'') {
            self.advance()?;
        }
        self.advance()?;

        ok_tok!(self, Character)
    }
    fn string(&mut self) -> ScanResult {
        while !r#match!(self, '"') {
            self.advance()?;
        }
        self.advance()?; // consume end quote

        ok_tok!(self, String)
    }
}
