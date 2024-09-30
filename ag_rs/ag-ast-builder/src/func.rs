use crate::{expr::Expr, types::Type, void};

pub struct Function {
    name: String,
    ret_type: Type,
    body: Vec<Expr>,
}
impl Function {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ret_type: void!(),
            body: Vec::new(),
        }
    }
    pub fn ret(mut self, ret_type: Type) -> Function {
        self.ret_type = ret_type;
        self
    }
    pub fn append_body_ref<T>(&mut self, body: T)
    where
        T: Into<Vec<Expr>>,
    {
        let mut body: Vec<Expr> = body.into();
        self.body.append(&mut body);
    }
    pub fn append_body<T>(mut self, body: T) -> Function
    where
        T: Into<Vec<Expr>>,
    {
        let mut body: Vec<Expr> = body.into();
        self.body.append(&mut body);
        self
    }
}
