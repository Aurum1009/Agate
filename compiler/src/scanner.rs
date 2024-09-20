pub struct Scanner<'src> {
    source: &'src String,
    chars: Vec<char>,
}
impl<'src> Scanner<'src> {
    pub fn new<'s: 'src>(source: &'s String) -> Self {
        Self {
            source,
            chars: source.chars().collect(),
        }
    }
}
