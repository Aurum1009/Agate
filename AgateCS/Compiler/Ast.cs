using AgateCS.Utils;

namespace AgateCS.Compiler
{
    namespace Ast
    {
        public interface IOptimize { }
        public interface IValidate
        {
            public List<string> Validate();
        }
    }
}

