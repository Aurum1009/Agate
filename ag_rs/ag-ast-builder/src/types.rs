#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TypeTag {
    Int,
    Uint,
    Float,
    Dec,
    String,
    Char,
    List(Box<Type>),
    Tuple(Box<Vec<Type>>),
    Union(Box<Vec<TypeTag>>),
    #[default]
    Any,
    Void,
    UserDefined {
        name: Box<String>,
        supertype: Option<Box<String>>,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Type {
    pub is_nullable: bool,
    pub tag: TypeTag,
}
impl Type {
    pub fn new(is_nullable: bool, tag: TypeTag) -> Self {
        Self { is_nullable, tag }
    }
}

mod macros {
    #[macro_export]
    macro_rules! void {
        () => {
            Type::new(false, ::ag_ast_builder::types::TypeTag::Void)
        };
    }
    #[macro_export]
    macro_rules! any {
        () => {};
    }
}
