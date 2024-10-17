using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AgateCS.Compiler.Ast
{
    public class ModuleAst(string name)
    {
        public string Name = name;

        public Dictionary<string, ModuleAst> Modules = [];
        public Dictionary<string, FunctionAST> Functions = [];
        public Dictionary<string, InterfactAst> Interfaces = [];

        public IAstError AddFunction(FunctionAST fn)
        {
            string name = fn.Name;
            if (Modules.ContainsKey(name) || Interfaces.ContainsKey(name) || !Functions.TryAdd(name, fn))
            {
                return new AlreadyExistsError(name);
            }
            else
            {
                return new FunctionOk();
            }
        }
    }

}
