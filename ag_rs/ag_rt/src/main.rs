use std::env::args;

mod io;
mod thread;

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() == 0 {
        eprintln!("Expected 1 argument [PATH], found []");
        return;
    }
}

pub struct VM {}
