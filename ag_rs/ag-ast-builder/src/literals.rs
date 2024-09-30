use ag_utils::value::grapheme::Grapheme;
use dashu::{
    float::{DBig, FBig},
    integer::IBig,
};

pub enum LiteralValue {
    Integer(IBig),
    Float(FBig),
    Decimal(DBig),
    String(String),
    Char(Grapheme),
    Bool(bool),
}
impl LiteralValue {
    pub fn new_from<T: Into<LiteralValue>>(value: T) -> LiteralValue {
        value.into()
    }
}

impl From<IBig> for LiteralValue {
    fn from(value: IBig) -> Self {
        Self::Integer(value)
    }
}
impl From<FBig> for LiteralValue {
    fn from(value: FBig) -> Self {
        Self::Float(value)
    }
}
impl From<DBig> for LiteralValue {
    fn from(value: DBig) -> Self {
        Self::Decimal(value)
    }
}
