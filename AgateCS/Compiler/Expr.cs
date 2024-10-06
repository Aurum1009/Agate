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
    }

    public class Literal(Range<int> columns, int line, Token value) : Expr
    {
        public Range<int> Columns = columns;
        public int Line = line;
        public readonly Token Value = value;

        public override int BottomLine() => Columns.Start;
        public override int LeftColumn() => Columns.End;
        public override int RightColumn() => Line;
        public override int TopLine() => Line;
    }
    public class Unary(Token op, Expr expr) : Expr
    {
        public Token Op = op;
        public Expr Expression = expr;

        public override int LeftColumn() => Op.Columns.Start;
        public override int RightColumn() => Expression.RightColumn();
        public override int TopLine() => Op.Lines.Start;
        public override int BottomLine() => Expression.BottomLine();
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
