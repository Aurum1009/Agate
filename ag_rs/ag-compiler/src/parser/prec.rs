#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Precedence {
    /// 1, ident, call, (x)
    Primary,
    /// list[idx]
    Index,
    /// x.y
    Get,
    /// x ** y
    Exponent,
    /// x * y, x / y, x % y
    Factor,
    /// x + y, x - y
    Term,
    /// x..y, x..=y, x..., ..x
    Range,
    /// x > y, x < y, x >= y, x <= y
    Cmp,
    /// x == y, x != y, x is T
    Eq,
    /// x & y, x | y, x ^ y
    Bitwise,
    /// x and y, x or y
    Logical,
    /// x = y, x += y, x -= y, x *= y, x /= y, x %= y, x **= y, x &= y, x |= y, x ^= y
    Assignment,

    None = 255,
}
