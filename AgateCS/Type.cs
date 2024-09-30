using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AgateCS
{

    public enum TypeTag
    {
        Int, 
        Uint,
        Float,
        String,
        Char,
        Union,
        Any
    }
    public sealed class Type(bool? isNullable, TypeTag? tag, string? name)
    {
        public TypeTag Tag = tag ?? TypeTag.Any;
        public string? Name = name;
        public List<Type> SubTypes = [];
        public Type? SuperType = null;
        public bool IsNullable = isNullable ?? false;

        public void SetName(string name)
        {
            this.Name = name;
        }
        public void SetSyperType(Type superType)
        {
            this.SuperType = superType;
        }
        public void AddSubType(Type type)
        {
            this.SubTypes.Add(type);
        }
        public void AddSubTypes(List<Type> types)
        {
            foreach (Type type in types)
            {
                this.SubTypes.Add(type);
            }
        }
    }
}
