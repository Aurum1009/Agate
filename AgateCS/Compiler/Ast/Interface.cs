using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AgateCS.Compiler.Ast
{
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
}
