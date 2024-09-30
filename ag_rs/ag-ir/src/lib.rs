use ast::ProjectAST;

pub mod ast;

pub trait Target
where
    Self: Sized,
{
}

impl Target for ProjectAST {}

pub trait Lower<T>
where
    T: Sized + Target,
{
    fn lower(self) -> T;
}
