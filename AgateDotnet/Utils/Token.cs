using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Utils
{
    public struct Range<T>(T start, T end)
    {
        public T Start { get; set; } = start;
        public T End { get; set; } = end;
    }
    public enum TokenType
    {
        StringLiteral,
        CharLiteral,
        IntegerLiteral,
        BinaryLiteral,
        OctalLiteral,
        HexadecimalLiteral,
        FloatLiteral,
        ScientificNumLiteral,
        True,
        False,
        Null,
        Identifier,
        // more
    }
    public class Token(string lexeme, TokenType type, Range<int> lines, Range<int> columns)
    {
        public readonly string Lexeme = lexeme;
        public readonly TokenType Type = type;
        public readonly Range<int> Lines = lines;
        public readonly Range<int> Columns = columns;

    }
}
