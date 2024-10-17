macro_rules! define_opcode {
    ($($name: ident, $value: literal), *) => {
        $(
            pub const $name: u8 = $value;
        )*
    };

}
macro_rules! define_module {
    ($module: ident) => {
        pub mod $module {}
    };
    ($module: ident, $($name: ident, $value: literal), *) => {
        pub mod $module {
            define_opcode!($($name, $value)*)
        }
    };
}

define_opcode!(PRINT, 0);
// common operations
define_opcode!(ADD, 1, SUB, 2, MUL, 3, DIV, 4, REM, 5);
// logical operations
define_opcode!(LOG_AND, 6, LOG_OR, 7, LOG_NOT, 8);
// bitwise operations
define_opcode!();