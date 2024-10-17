using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AgateCS.Compiler.Ast
{
    public interface IAstError
    {

        string Msg();
    }

    public class AlreadyExistsError(string name) : IAstError
    {
        public readonly string ItemName = name;

        public string Msg() => $"An item with the name {ItemName} already exists in this scope";
    }
    public class UnknownTypeName(string name) : IAstError
    {
        public string TypeName { get; } = name;

        public string Msg() => $"Unkown type name {TypeName}";
    }
    public class NonPrimitiveTypeCast(string originType, string castedType) : IAstError
    {
        public readonly string OriginType = originType;
        public readonly string CastedType = castedType;

        public string Msg() => $"Cannot perform non-primitive cast from {OriginType} to {CastedType}";
    }
    public class UnknownMethod(string typeName, string methodName)
    {
        public readonly string TypeName = typeName;
        public readonly string MethodName = methodName;

        public string Msg() => $"Type {TypeName} contains no method named {MethodName}";
    }
    public class UnknownAssociatedFunction(string typeName, string funcName)
    {
        public readonly string TypeName = typeName;
        public readonly string FunctionName = funcName;

        public string Msg() => $"Type {TypeName} contains no associated function or method named {FunctionName}";
    }
    public class FunctionOk() : IAstError
    {
        public string Msg() => "Function succeeded";
    }
}
