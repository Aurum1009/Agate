use crate::{repr::SimpleFileRepr, scanner::Scanner};

pub struct Parser<'src, 'repr> {
    source: &'src String,
    scanner: Scanner<'src>,
    repr: &'repr mut SimpleFileRepr,
}
