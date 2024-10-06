namespace AgateCS.Compiler
{
    public interface IOptimize { }
    public interface IValidate
    {
        public List<string> Validate();
    }

    public class FunctionAST(string Name, List<Expr> Body, List<(string, Type)> Args, List<string>? Generics = null) : IValidate, IOptimize
    {
        public readonly string Name = Name;
        protected Type ReturnType = Type.Void();
        public readonly List<(string, Type)> Arguments = Args;
        private readonly List<string>? Generics = Generics;
        private readonly List<Expr> Body = Body;

        public void SetReturn(Type ReturnType)
        {
            this.ReturnType = ReturnType;
        }

        public List<string> Validate()
        {
            return [];
        }
    }
    public class SignatureAST(string name, Type type, List<(string, Type)> parameters, List<string>? generics = null)
    {
        public string Name = name;
        public Type ReturnType = type;
        public List<(string, Type)> Parameters = parameters;
        public List<string>? Generics = generics;
    }
    public class InterfactAst(string name, List<SignatureAST> signatures, List<String>? generics = null) : IValidate, IOptimize
    {
        public string Name = name;
        public List<SignatureAST> Functions = signatures;
        public List<string>? Generics = generics;

        public List<string> Validate()
        {
            return [];
        }
    }
    public class ModuleAst(string name)
    {
        public string Name = name;

        public Dictionary<string, ModuleAst> Methods = [];
        public Dictionary<string, FunctionAST> Functions = [];
        public Dictionary<string, InterfactAst> Interfaces = [];
    }
}

