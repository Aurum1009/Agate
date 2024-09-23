#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    // groups
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    // operators - logical & comparison
    EqualEqual,   // ==
    BangEqual,    // !=
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=
    // operators - mathematic
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Percent, // %
    Power,   // **
    // operators - mathematical assignment
    PlusEqual,    // +=
    MinusEqual,   // -=
    StarEqual,    // *=
    SlashEqual,   // /=
    PercentEqual, // %=
    PowerEqual,   // **=
    // operators - bitwise
    BitAnd,     // &
    BitOr,      // |
    BitXor,     // ^
    ShiftLeft,  // <<
    ShiftRight, // >>
    // operators - bitwise assignment
    BitAndEqual, // &=
    BitOrEqual,  // |=
    BitXorEqual, // ^=
    // operators - logical
    And,  // and
    Or,   // or
    Bang, // !
    // operators - other
    Newline,   // \n
    Semicolon, // ;
    Equal,     // =
    FatArrow,  // =>
    //  variables
    Let,    // let
    Var,    // var
    Global, // global
    Mut,    // mut
    Colon,  // :
    // keywords - control flow
    If,     // if
    Unless, // until
    While,  // while
    Until,  // until
    For,    // for
    In,     // in
    Loop,   // loop
    // functions
    Func,        // func
    CapitalSelf, // Self
    _Self,       // self
    RetArrow,    // ->
    // classes
    Class,    // class
    Inherits, // <:
    Dot,      // .
    // arrays
    ExclusiveRange, // ..
    InclusiveRange, // ..=
    // literals & others
    Pub,        // pub
    Identifier, // ident
    Character,  // 'c'
    String,     // "string..."
    Integer,    // 1
    Decimal,    // 1.0
    True,       // true
    False,      // false
    Null,       // null
    Pipe,       // |>

    EOF,
}
