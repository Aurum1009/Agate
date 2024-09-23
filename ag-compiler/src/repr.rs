pub mod expr;
pub mod items;
pub mod utils;

pub trait CanParse
where
    Self: Sized,
{
    fn needs_link(&self) -> bool;
}

pub struct ModuleRepr {}
impl ModuleRepr {}
impl CanParse for ModuleRepr {
    fn needs_link(&self) -> bool {
        false
    }
}

pub struct ProjectRepr {}
impl ProjectRepr {
    pub fn empty() -> Self {
        Self {}
    }
    pub fn from_mod(module: ModuleRepr) -> Self {
        Self {}
    }
}
impl CanParse for ProjectRepr {
    fn needs_link(&self) -> bool {
        false
    }
}
