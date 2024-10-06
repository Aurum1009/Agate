use clap::{Arg, ArgAction, Command};
use std::process::exit;

pub const MAJOR: usize = 0;
pub const MINOR: usize = 1;
pub const PATCH: usize = 0;
pub const VER_SLICE: &str = "0.1.0";

fn make_command() -> clap::Command {
    Command::new("agate")
        .about("The agate programming language compiler and runtime")
        .arg(
            Arg::new("path")
                .help("Path to the Agate source file")
                .required(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::new("release")
                .long("rel")
                .short('r')
                .help("Compile in release mode instead of debug mode")
                .action(ArgAction::SetTrue)
                .global(true),
        )
        .version(VER_SLICE)
}

fn main() {
    let command = make_command();

    let matches = command.get_matches();

    if let Some(path) = matches.get_one::<String>("path") {
        compile(path.clone(), matches.get_flag("release"));
    } else {
        eprintln!("Source file is required to compile.");
        exit(1);
    }
}

fn compile(path: String, in_release: bool) {}
