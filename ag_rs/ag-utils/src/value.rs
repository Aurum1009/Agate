use super::objects::Object;
use ::{
    dashu::{float::FBig, Decimal, Integer, Natural},
    parking_lot::RwLock,
    std::sync::Arc,
};

#[derive(Debug, Clone)]
/// # The `Value` enum
///
/// All values passed around in Agate are part of this enum.
///
/// ## Why does the `Value` enum have two float types?
///
/// Agate supports two native decimal number types: a `float` and a `dec`. the default is a `dec`,
/// as is is more precise than a `float`, but if you want to use a float, suffixing a number with
/// 'f' is good enough. So, `1.0f` is a float, but `1.0` and `1.0d` is a decimal.
///
/// We recommend using a `dec` whenever you are not doing scientific calculations or are in a
/// performance critical environment.
pub enum Value {
    /// A abstract precision integer. This makes use of the `dashu::integer::IBig` type through the `dashu::Integer` alias.
    Int(Box<Integer>),
    /// A unsigned abstract precision integer. This makes use of `dashu`'s Natural type, which is an alias for `dashu::integer::UBig`
    Uint(Box<Natural>),
    /// An abstract precision float. The usage of this type is only recommended when you need high performance or when you are
    /// performing precise scientific calculations. Otherwise, it is better to use a Decimal number.
    Float(Box<FBig>),
    /// A decimal number for general-purpose use. This makes use of `dashu::Decimal`, which is an accurate number with
    /// regular, intuitive rounding(to the nearest with 0.5 rounding to 1)
    Decimal(Box<Decimal>),
    /// A character literal, represented by a single unicode grapheme, with help from `unicode-segmentation`. See
    /// `Grapheme` for more details.
    Char(Box<grapheme::Grapheme>),
    /// A boolean value, can be `true` or `false`, represented by a Rust `bool`.
    Bool(bool),
    /// An object value, inside of an `RwLock`, allowing for safe mutation, inside of an `Arc` for memory management.
    /// See `object::Object` for more details.
    Object(Arc<RwLock<Object>>),
}
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(i1), Self::Int(i2)) => i1 == i2,
            (Self::Uint(u1), Self::Uint(u2)) => u1 == u2,
            (Self::Float(f1), Self::Float(f2)) => f1 == f2,
            (Self::Decimal(d1), Self::Decimal(d2)) => d1 == d2,

            (Self::Char(c1), Self::Char(c2)) => c1 == c2,
            (Self::Bool(b1), Self::Bool(b2)) => b1 == b2,

            (Self::Object(o1), Self::Object(o2)) => o1.read().eq(&o2.read()),
            _ => false,
        }
    }
}

/// # Agate Graphemes
///
/// This module contains the definition and all of the utilities implemented for Grapheme. Grapheme uses a string slice
/// to represent an entire Unicode grapheme efficiently. These are different from Rust's `char`, because a `char` only
/// represents a single Unicode Scalar Value, meaning that something like `'y̆'` is not one character in Rust's representation,
/// but two: `['y', '\u{0306}']`. This is a problem, because to us, `'y̆'` is one character, and we usually do not write code that
/// can handle things like `'\u{0306}'` correctly, leading to false error conditions because a type is represented in an unintuitive way.
pub mod grapheme {
    use ::{std::str::FromStr, unicode_segmentation::UnicodeSegmentation};
    #[derive(Debug, PartialEq, Eq, Clone)]
    /// A single unicode grapheme, which is more compact than using String.
    pub struct Grapheme {
        /// The grapheme representation, as a boxed string slice. it is boxed because a `String` is needlessly large and a string
        /// slice needs to be boxed to avoid references(because they would not be `'static`)
        grapheme: Box<str>,
    }
    impl Grapheme {
        /// Attempts to get a grapheme from a string slice by converting it into a String and using `Grapheme::from_str`.
        /// This will return None if the string is empty. If the string has more than one grapheme, these are ignored.
        pub fn from_slice(str: &str) -> Option<Self> {
            let string = str.to_string();
            Self::from_string(string)
        }
        /// Attempts to get a grapheme from a `String`. This will return None if the string is empty, and if the string has
        /// more than one grapheme, they are disregarded.
        pub fn from_string(str: String) -> Option<Self> {
            let mut graphemes = str.graphemes(true);
            graphemes.next().map(|grapheme| Self {
                grapheme: grapheme.to_string().into_boxed_str(),
            })
        }
        /// Get the internal `str` from a Grapheme. This can be for printing purposes, or just for representation purposes.
        pub fn get_slice(&self) -> &str {
            &self.grapheme
        }
    }
    impl FromStr for Grapheme {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Self::from_slice(s)
                .ok_or_else(|| "Cannot derive a character from an empty string".to_string())
        }
    }
    impl TryFrom<String> for Grapheme {
        type Error = String;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            Self::from_string(value)
                .ok_or_else(|| "Cannot derive a character from an empty string".to_string())
        }
    }
    impl From<Grapheme> for Box<str> {
        fn from(value: Grapheme) -> Self {
            value.grapheme
        }
    }
    impl From<Box<str>> for Grapheme {
        fn from(value: Box<str>) -> Self {
            Self { grapheme: value }
        }
    }
}
