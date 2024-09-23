#[derive(Debug, PartialEq, Clone)]
/// # Agate's Object enum
///
/// This enum holds objects, which are too big to hold in the Value enum. These include classes, enums, and functions.
/// These objects are typically large, and not efficient to use outside of smart pointers like `Rc` and `Arc` because of their size.
pub enum Object {}
