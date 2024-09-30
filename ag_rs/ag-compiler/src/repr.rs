use items::*;

use crate::ProjectType;

pub(crate) mod expr;
pub(crate) mod items;
pub(crate) mod utils;

pub(crate) trait CanParse
where
    Self: Sized,
{
    type Result: Sized;

    fn needs_external_link(&self) -> bool;
    fn add_fn(&mut self, function: FunctionRepr) -> Result<(), String>;
    fn add_class(&mut self, class: ClassRepr) -> Result<(), String>;
    fn add_mod(&mut self, function: FunctionRepr) -> Result<(), String>;
    fn add_import(&mut self, import: ImportRepr) -> Result<(), String>;
    fn add_alias(&mut self, alias: AliasRepr) -> Result<(), String>;
}

pub(crate) struct ProjectRepr {
    project_type: ProjectType,
}
impl ProjectRepr {
    pub fn empty(project_type: ProjectType) -> Self {
        Self { project_type }
    }
}
impl ag_ir::Lower<ag_ir::ast::ProjectAST> for ProjectRepr {
    fn lower(self) -> ag_ir::ast::ProjectAST {
        use ag_ir::ast::ProjectAST;
        let ast = ProjectAST {};
        ast
    }
}
