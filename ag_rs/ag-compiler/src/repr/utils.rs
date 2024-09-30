pub mod types {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct TypeRepr {
        pub base: TypeReprBase,
        pub is_nullable: bool,
    }
    impl TypeRepr {
        pub fn new(base: TypeReprBase, is_nullable: bool) -> Self {
            Self { base, is_nullable }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum TypeReprBase {
        Any,
        Int,
        Uint,
        Float,
        Decimal,
        String,
        Char,
        List(Box<TypeRepr>),
        Custom {
            name: Box<String>,
            supertype: Option<Box<String>>,
        },
    }
}

#[derive(Debug)]
pub struct PathRepr {
    pub prefix: PathPrefix,
    pub segments: Vec<String>,
    pub items: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PathPrefix {
    Super,
    Self_,
    Module(String),
}
