namespace AgateCS.Compiler
{
    public enum TypeKind
    {
        Class,
        Interface,
        Struct,
        Enum,
    }
    public enum TypeTag
    {
        Void,
        Any,

        Byte, // u8
        I32,
        I64,
        U32,
        U64,
        Int,  // AP int
        Uint, // AP uint
        F32,
        F64,

        Union,
        Tuple,
    }

    public class Type(TypeTag Tag, bool IsNullable)
    {
        public bool IsNullable = IsNullable;
        public TypeTag Tag = Tag;
        public List<string>? Generics;
        /// <summary>
        /// Types such as members of a union type or members of a tuple type
        /// </summary>
        public List<Type>? SubTypes;

        public static Type Void()
        {
            return new(TypeTag.Void, false);
        }
    }
    public class TypeInternals(TypeKind Kind, string Name)
    {
        public TypeKind TypeKind = Kind;
        public string Name = Name;
    }
}
