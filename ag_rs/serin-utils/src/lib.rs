pub mod instructions;
pub mod object;
pub mod value;
mod lazy;

pub fn asm() {
    let asm = unsafe { std::arch::asm!("") };
    asm
}
