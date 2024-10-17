using AgateCS.Scanner;
using AgateCS.Utils;

namespace AgateCS.Compiler
{
    public abstract class Expr
    {
        public abstract int LeftColumn();
        public abstract int RightColumn();
        public abstract int TopLine();
        public abstract int BottomLine();
        /// <summary>
        /// 
        /// </summary>
        /// <returns>Returns null if the type is invalid.</returns>
        public abstract Type? TypeOf();
    }
    public abstract class StmtExpr : Expr
    {
        public required List<string> Attributes;
    }
    public sealed class Literal(Range<int> columns, int line, Token value) : Expr
    {
        public Range<int> Columns = columns;
        public int Line = line;
        public readonly Token Value = value;

        public override int BottomLine() => Columns.Start;
        public override int LeftColumn() => Columns.End;
        public override int RightColumn() => Line;
        public override int TopLine() => Line;
        public override Type? TypeOf() => Value.Type switch {
            TokenType.IntegerLit => new Type(TypeTag.Int, false),
            TokenType.FloatLit => new(TypeTag.Float, false),
            TokenType.StringLit => new Type(TypeTag.String, false),
            _ => null,
        };
    }
    public class Unary(Token op, Expr expr) : Expr
    {
        public Token Op = op;
        public Expr Expression = expr;

        public override int LeftColumn() => Op.Columns.Start;
        public override int RightColumn() => Expression.RightColumn();
        public override int TopLine() => Op.Lines.Start;
        public override int BottomLine() => Expression.BottomLine();
        public override Type? TypeOf() => Expression.TypeOf();
    }
    public class Binary(Expr left, Token op, Expr right) : Expr
    {
        public Expr Left = left;
        public Token Op = op;
        public Expr Right = right;

        public override int LeftColumn() => Left.LeftColumn();
        public override int RightColumn() => Right.RightColumn();
        public override int TopLine() => Left.TopLine();
        public override int BottomLine() => Right.BottomLine();
    }
}
