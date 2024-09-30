//! # Agate's Compiler module
//!
//! This module contains the parsers, lowering operations and utilities to parse an Agate source file
//! into a intermediate representation called `Agile`. `Agile` is defined in the `utils` crate, because
//! it needs to be used in multiple places.
//!
//! The main thing this module contains is the `compile` function. It takes a source file(as a string), and
//! does all of the compilation, project linking & validation process. SFR optimization also happens here.
//!
//! This module's output does its best to be portable for all codegen options.
//! <br><br>
//!
//! The logic behind the linking working with the SFR, or Simple File Representation(a *very* high level AST),
//! is that if the SFR contains extern modules(aka `mod my_mod` where the brackets are missing), a `Includer`
//! can be created and replace these definitions with the fully-compiled & included file.
//!
//! This process can and does become recursive. (And yes, the produced files, whether `.agbc`, `.avian`, `.wat` or
//! `.wasm` are large, but there is only one of them and it contains the entire program. LTO is not used, but for example,
//! in `std`, if you do not have an import to a module it gets forgotten.)
#![deny(
    missing_docs,
    clippy::empty_docs,
    clippy::missing_docs_in_private_items,
    rustdoc::missing_crate_level_docs,
    rustdoc::suspicious_doc_comments
)]

use std::process::exit;

use ag_ir::{ast::ProjectAST, Lower};
// reexport of optlevel
pub use ag_utils::OptLevel;

use ag_utils::Profile;

use self::{
    opt::Optimizer,
    parser::Parser,
    repr::{CanParse, ProjectRepr},
};

/// Determines how the current project will be built
pub enum ProjectType {
    /// Will create a `.aglib` file when built
    Library,
    /// Will create a `.agbc` or `.agexec` file when built
    Project,
}

mod opt;
/// ## The `compiler::parser` Module
///
/// This module contains all code for the parser. The parser takes in a String (and some args for
/// configuration) and will output a SimpleFileRepr.
mod parser;
mod repr;
mod scanner;
mod token;
mod token_type;

/// Compiles the given source into a SimpleProjectRepr. If the file needs to be linked,
/// a linker will be initialized and the source will be linked into a new module.
pub fn compile(
    name: String,
    source: &String,
    opt_level: OptLevel,
    profile: Profile,
    project_type: ProjectType,
) -> ProjectAST {
    let mut main = ProjectRepr::empty(project_type);
    todo!()
    /*
    let had_parse_err = {
        let mut parser = Parser::new(name, &source, &mut main, true);

        parser.parse()
    };
    if had_parse_err {
        eprintln!("Parse error occurred, compilation ceasing.");
        exit(1); // it is safe to use exit() because the parser is dropped already
    }

    if main.needs_external_link() {}

    let opt: Optimizer<'_> = Optimizer::new(&mut main, opt_level, profile);
    opt.optimize();

    return main.lower();
    */
}
