using System.Text;
using AgateCS.Scanner.ScanUtils;
using AgateCS.Utils;
using static AgateCS.Scanner.TokenType;

namespace AgateCS.Scanner
{
    namespace ScanUtils
    {
        public interface IScanResult
        {
            public string Message();
        }

        public class InvalidChar(char c, string reason) : IScanResult
        {
            public string Reason = reason;
            public readonly char Character = c;

            public override string ToString()
            {
                return $"{Reason} character '{Character}'";
            }
            public string Message()
            {
                return ToString();
            }
        }

        public readonly struct UnexpectedEnd(string msg) : IScanResult
        {
            public readonly string Msg = msg;

            public string Message()
            {
                return Msg;
            }
        }
    }
    
    public static class Scanner
    {
        private static string Source = "";

        public static void ReInit(string source)
        {
            Source = source;
            Line = 1;
            Start = 0;
            Current = 0;
            Columns.Start = 0;
            Columns.End = 0;
        }

        private static int Line = 1;
        private static int Start = 0;
        private static readonly Range<int> Columns = new(0, 0);
        private static int Current = 0;

        private static void SkipWhiteSpace()
        {
            while (true)
            {
                switch (Source[Current])
                {
                    case ' ':
                    case '\r':
                    case '\t':
                        break;
                    case '#':
                        while (!IsAtEnd())
                        {
                            Advance();
                            if (Peek() == '\n')
                            {
                                break;
                            }
                        }
                        break;
                    default:
                        return;
                }
            }
        }
        private static char MoveForward()
        {
            Columns.End++;
            return Source[Current++];
        }
        private static char Advance() => IsAtEnd() ? '\0' : MoveForward();
        private static char Peek(int count = 0) => (Current + 1 + count >= Source.Length) ? '\0' : Source[Current + 1 + count];
        private static bool Match(char c)
        {
            if (Peek() == c)
            {
                Advance();
                return true;
            }
            return false;
        }
        private static Token MakeToken(TokenType type) => new(Columns, new(Line, Line), type, Source[Start..Current]);
        private static Token MakeToken(TokenType baseType, char match, TokenType matchType) => MakeToken(Match(match) ? matchType : baseType);
        private static bool IsAtEnd() => Current >= Source.Length;

        public static IScanResult ScanToken()
        {
            SkipWhiteSpace();

            Start = Current;
            Columns.Start = Columns.End;

            char c = Advance();

            IScanResult result =  c switch
            {
                '(' => MakeToken(LeftParen),
                ')' => MakeToken(RightParen),
                '{' => MakeToken(LeftBrace),
                '}' => MakeToken(RightBrace),
                '[' => MakeToken(LeftBracket),
                ']' => MakeToken(RightBracket),
                '!' => MakeToken(Bang, '=', BangEqual),
                '=' => Match('>') ? MakeToken(FatArrow) : MakeToken(Equal, '=', EqualEqual),
                '+' => MakeToken(Plus, '=', PlusEqual),
                '-' => Match('>') ? MakeToken(RetArrow) : MakeToken(Minus, '=', MinusEqual),
                '*' => Match('*') ? MakeToken(Power, '*', PowerEqual) : MakeToken(Star, '=', StarEqual),
                '/' => MakeToken(Slash, '=', SlashEqual),
                '%' => MakeToken(Percent, '=', PercentEqual),
                '&' => MakeToken(BitAnd, '=', BitAndEqual),
                '|' => Match('>') ? MakeToken(Pipe) : MakeToken(BitOr, '=', BitOrEqual),
                '^' => MakeToken(BitXor, '=', BitXorEqual),
                '>' => MakeToken(Greater, '=', GreaterEqual),
                '<' => Match(':') ? MakeToken(Inherits) : MakeToken(Less, '=', LessEqual),
                '\n' => NewLine(),
                ';' => MakeToken(Semicolon),
                '?' => MakeToken(Question),
                ':' => Match('=') ? MakeToken(ColonEqual) : MakeToken(Colon, ':', ColonColon),
                '.' => Match('.') ? MakeToken(ExclusiveRange, '=', InclusiveRange) : MakeToken(Dot),
                '"' => String(),
                '\'' => Char(),


                _ when char.IsDigit(c) => Number(),
                _ when char.IsLetter(c) || c == '_' => Ident(),
                _ => new InvalidChar(c, "Unkown"),
            };

            if (AgateCS.Config.DebugScan && result is Token token)
            {
                Console.WriteLine($"|- Debug.Scan: Produced token {token.Message()}");
            }
            return result;
        }

        private static Token NewLine()
        {
            Line++;
            Columns.End = 0;
            return MakeToken(Newline);
        }
        private static IScanResult Char()
        {
            if (IsAtEnd() || Peek() == '\'' || Peek() == '\n')
            {
                return new UnexpectedEnd("Unended or invalid character literal");
            }

            char c = Advance();
            
            if (c == '\\')
            {
                switch(Advance())
                {
                    case 'n':
                        c = '\n';
                        break;
                    case 't':
                        c = '\t';
                        break;
                    case 'r':
                        c = '\r';
                        break;
                    case '\\':
                        c = '\\';
                        break;
                    case '\'':
                        c = '\'';
                        break;
                    case '0':
                        c = '\0';
                        break;
                    default:
                        return new InvalidChar(c, "Invalid escape");

                }
            }

            if (Peek() != '\'')
            {
                return new UnexpectedEnd("Character literal is too long");
            }

            Advance();

            return new Token(Columns, new(Line, Line), Character, Source[Start..Current], c.ToString());
        }
        private static IScanResult String()
        {
            int StartLine = Line;
            StringBuilder builder = new();
            while (Peek() != '"' && !IsAtEnd())
            {
                char c = Advance();

                if (c == '\\')
                {
                    char next = Advance();
                    builder.Append(next switch
                    {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '"' => '"',
                        '\'' => '\'',
                        '0' => '\0',
                        _ => next,
                    });
                }
                else
                {
                    builder.Append(c);
                }
            }

            if (IsAtEnd()) return new UnexpectedEnd("Unended string literal");
            else Advance();

            return new Token(Columns, new(StartLine, Line), StringLit, Source[Start..Current], builder.ToString());
        }

        private static Token Ident()
        {
            while ((char.IsDigit(Peek()) || char.IsLetter(Peek()) || Peek() == '_') && !IsAtEnd())
            {
                Advance();
            }

            string text = Source[Start..Current];
            return text switch
            {
                "and" => MakeToken(And),
                "or" => MakeToken(Or),
                "let" => MakeToken(Let),
                "var" => MakeToken(Var),
                "global" => MakeToken(Global),
                "mut" => MakeToken(Mut),
                "if" => MakeToken(If),
                "unless" => MakeToken(Unless),
                "while" => MakeToken(While),
                "until" => MakeToken(Until),
                "loop" => MakeToken(Loop),
                "for" => MakeToken(For),
                "in" => MakeToken(In),
                "func" => MakeToken(Func),
                "class" => MakeToken(Class),
                "self" => MakeToken(Self),
                "Self" => MakeToken(CapitalSelf),
                "mod" => MakeToken(Mod),
                "pub" => MakeToken(Pub),
                "true" => MakeToken(True),
                "false" => MakeToken(False),
                "null" => MakeToken(Null),
                "is" => MakeToken(Is),
                _ => MakeToken(Identifier),
            };
        }
        private static Token Number()
        {
            while (char.IsDigit(Peek()) && !IsAtEnd())
            {
                Advance();
            }
            if (Peek() == '.' && char.IsDigit(Peek(1)))
            {
                Advance();
                while (char.IsDigit(Peek()) && !IsAtEnd())
                {
                    Advance();
                }
                return MakeToken(FloatLit);
            }
            else
            {
                return MakeToken(IntegerLit);
            }
        }
    }
}
