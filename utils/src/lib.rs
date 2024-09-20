//! # Agate Utilities
#![deny(
    missing_docs,
    clippy::empty_docs,
    clippy::missing_docs_in_private_items,
    rustdoc::missing_crate_level_docs,
    rustdoc::suspicious_doc_comments
)]

/// Agate's SemVer major version number
pub const MAJOR_VERSION: usize = 0;
/// Agate's SemVer minor version number
pub const MINOR_VERSION: usize = 1;
/// Agate's SemVer patch number
pub const PATCH: usize = 0;

/// The profile the compiler uses to perform certain optimizations and inclusions.
/// `Debug` will perform no optimizations, while `Release` will.
pub enum Profile {
    /// The release configuration of Agate. Allows optimizations to run & does not allow any
    /// code inside of a `dbg {}` to compile.
    Release,
    /// The debug configuration of Agate. Turns of optimizations for the compiler & allows
    /// code wrapped in a `dbg {}` to compile, in either the compiler or AVIAN.
    Debug,
}

/// Specifies what output the compiler should build
pub enum Target {
    /// Compile the Agate AST into Web Assembly
    WebAssembly,
    /// Compile the Agate AST into a binary bytecode/project format
    Bytecode,
    /// Compile the Agate AST into Agate's human-readable version of its bytecode, AVIAN
    Avian,
}

/// A global implementation of the main Args, but without the `clap` implementations
/// derived, and better information held.
pub struct GlobalArgs {
    path: String,
    opt_level: u8,
    profile: Profile,
    output: String,
    target: Target,
    recursive_opt: bool,
    run: bool,
}
