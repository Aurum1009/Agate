using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AgateCS.Compiler.Ast
{
    public class FunctionAST(string Name, List<Expr> Body, List<(string, Type)> Args, List<string>? Generics = null) : IValidate, IOptimize
    {
        public readonly string Name = Name;
        protected Type ReturnType = Type.Void();
        public readonly List<(string, Type)> Arguments = Args;
        private readonly List<string>? Generics = Generics;
        private readonly List<Expr> Body = Body;

        public FunctionAST SetReturn(Type ReturnType)
        {
            this.ReturnType = ReturnType;
            return this;
        }

        public List<string> Validate()
        {
            return [];
        }
    }

}
