using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using AgateCS.Scanner.ScanUtils;
using AgateCS.Utils;

namespace AgateCS.Scanner
{
    public sealed record Token(Range<int> columns, Range<int> lines, TokenType type, string lexeme, string? lit = null) : IScanResult
    {
        public Range<int> Columns = columns;
        public Range<int> Lines = lines;
        public TokenType Type = type;
        public string Lexeme = lexeme;
        public string? Literal = lit;

        public static Token Eof()
        {
            return new Token(new(0, 0), new(0, 0), TokenType.EOF, "");
        }

        public string Message()
        {
            return $"{Type} [{Lines.ToTokenString()}]:{Columns.Start}..{Columns.End}] \"{Lexeme}\"";
        }
    }

    public enum TokenType
    {
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

        // operators - mathematical
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
        ColonEqual,// :=
        FatArrow,  // =>

        //  variables
        Let,    // let
        Var,    // var
        Global, // global
        Mut,    // mut
        Colon,  // :

        // keywords - control flow
        If,     // if
        Unless, // unless
        While,  // while
        Until,  // until
        For,    // for
        In,     // in
        Loop,   // loop

        // functions
        Func,        // func
        RetArrow,    // ->

        // classes
        Class,    // class
        CapitalSelf, // Self
        Self,       // self
        Inherits, // <:
        Dot,      // .
                  // arrays
        ExclusiveRange, // ..
        InclusiveRange, // ..=

        // literals & others
        Question,   // ?
        Mod,        // mod
        Pub,        // pub
        Identifier, // ident
        Character,  // 'c'
        StringLit,     // "string..."
        IntegerLit,    // 1
        FloatLit,    // 1.0
        True,       // true
        False,      // false
        Null,       // null
        Pipe,       // |>
        Is,         // is
        ColonColon, // ::

        EOF,
    }
}
