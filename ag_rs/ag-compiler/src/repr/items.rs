pub type Lines = std::ops::Range<usize>;

mod func {
    use super::{super::expr::ExprRepr, Lines};
    use crate::repr::utils::types::TypeRepr;

    #[derive(Debug)]
    pub struct FunctionRepr {
        pub name: String,
        pub parameters: Vec<(String, TypeRepr)>,
        pub generics: Option<Vec<String>>,
        pub where_clauses: Option<Vec<WhereClauseRepr>>,
        pub body: Vec<ExprRepr>,
        pub lines: Lines,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct WhereClauseRepr {
        pub type_name: String,
        pub constraint: WhereConstraint,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum WhereConstraint {
        ImplementsInterface(String),
        HasConnectedFunction(String, Vec<TypeRepr>),
        HasProperty(String),
    }
}

mod class {
    use super::{
        super::{expr::ExprRepr, utils::types::TypeRepr},
        *,
    };
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct ClassRepr {
        pub name: String,
        pub superclass: Option<Vec<String>>,
        pub constructor: Option<FunctionRepr>,
        pub fields: HashMap<String, TypeRepr>,
        pub defaults: Vec<(String, ExprRepr)>,
        pub members: HashMap<String, MemberRepr>,
    }

    #[derive(Debug)]
    pub struct MemberRepr {
        pub name: String,
        pub generics: Option<Vec<String>>,
        pub self_usage: SelfUsage,
        pub args: Vec<(String, TypeRepr)>,
        pub ret_type: TypeRepr,
        pub body: Vec<ExprRepr>,
        pub lines: Lines,
    }

    #[derive(PartialEq, Eq, Debug, Clone, Copy)]
    pub enum SelfUsage {
        None,
        ImmutableSelf,
        MutableSelf,
    }
}

mod module {
    use std::collections::HashMap;

    use super::*;

    #[derive(Debug)]
    pub struct ModuleRepr {
        pub name: String,
        pub functions: HashMap<String, (bool, FunctionRepr)>,
        pub modules: HashMap<String, (bool, ModuleRepr)>,
        pub imports: Vec<(bool, ImportRepr)>,
        pub extern_modules: HashMap<String, (bool, ExternModuleRepr)>,
        pub classes: HashMap<String, (bool, ClassRepr)>,
        pub lines: Lines,
    }
}

mod extern_module {
    #[derive(Debug, Clone)]
    pub struct ExternModuleRepr {
        name: String,
        rename: Option<String>,
        from: Option<String>,
    }
}

mod import {
    use crate::repr::utils::PathRepr;

    #[derive(Debug)]
    pub struct ImportRepr {
        pub path: PathRepr,
        pub rename: Option<Vec<String>>,
    }
}

mod alias {
    use crate::repr::utils::PathRepr;

    #[derive(Debug)]
    pub struct AliasRepr {
        pub name: String,
        pub type_: PathRepr,
    }
}

mod enum_ {
    use crate::repr::{
        items::{Lines, MemberRepr},
        utils::types::TypeRepr,
    };
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct EnumRepr {
        pub name: String,
        pub generics: Option<Vec<String>>,
        pub variants: HashMap<String, (usize, Option<Vec<TypeRepr>>)>,
        pub members: HashMap<String, MemberRepr>,
        pub lines: Lines,
    }
}

pub use self::{alias::*, class::*, extern_module::*, func::*, import::*, module::*};
