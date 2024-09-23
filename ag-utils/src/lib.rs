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

/// # `agate::utils::objects`
pub mod objects;
/// # `agate::utils::value`
///
/// This module contains the `Value` enum as well as definitions for some helpful functions. For object definitions and helpers, see `agate::utils::objects`
pub mod value;
