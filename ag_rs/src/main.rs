use clap::{Arg, ArgAction, Command};

fn main() {
    let mut command = Command::new("agate")
        .about("Agate programming language compiler and runtime")
        .subcommand(
            Command::new("compile")
                .arg(
                    Arg::new("path")
                        .help("Path to the Agate source file")
                        .required(true)
                        .value_name("FILE"),
                )
                .arg(
                    Arg::new("run")
                        .short('r')
                        .long("run")
                        .help("Automatically run the compiled program after compilation")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("release")
                        .long("release")
                        .help("Compile in release mode instead of debug mode")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("test")
                .about("Test a WebAssembly(WASM) file")
                .arg(
                    Arg::new("path")
                        .help("Path to the WASM or WAT file")
                        .required(true)
                        .value_name("FILE"),
                ),
        )
        .subcommand(
            Command::new("run")
                .about("Run a build Agate bytecode or assembly file")
                .arg(
                    Arg::new("path")
                        .help("Path to the Agate bytecode/asm file")
                        .required(true)
                        .value_name("FILE"),
                ),
        )
        .arg(
            Arg::new("path")
                .help("Path to the Agate source file (default action being 'compile')")
                .required(false)
                .value_name("FILE"),
        )
        .arg(
            Arg::new("run")
                .short('r')
                .long("run")
                .help("Automatically run the compiled program after compilation")
                .action(ArgAction::SetTrue)
                .global(true),
        )
        .arg(
            Arg::new("release")
                .long("release")
                .help("Compile in release mode instead of debug mode")
                .action(ArgAction::SetTrue)
                .global(true),
        )
        .version("0.1.0");

    let matches = command.clone().get_matches();

    if let Some(path) = matches.get_one::<String>("path") {
        let run = matches.get_flag("run");
        let rel = matches.get_flag("release");

        compile(path, run, rel);
    } else {
        match matches.subcommand() {
            Some(("compile", sub_matches)) => {
                let path = match sub_matches.get_one::<String>("path") {
                    Some(p) => p,
                    None => unreachable!(),
                };
                let run = sub_matches.get_flag("run");
                let rel = sub_matches.get_flag("release");
                compile(path, run, rel);
            }
            Some(("test", sub_matches)) => {
                let path = match sub_matches.get_one::<String>("path") {
                    Some(p) => p,
                    None => unreachable!(),
                };
                test_wasm(path);
            }
            Some(("run", sub_matches)) => {
                let path = match sub_matches.get_one::<String>("path") {
                    Some(p) => p,
                    None => unreachable!(),
                };
                run_bytecode(path);
            }
            _ => {
                eprintln!("No valid subcommand or file path was provided");
                println!("{}", command.render_help());
            }
        }
    }
}

fn compile(path: &str, run: bool, in_release: bool) {
    todo!()
}
fn test_wasm(path: &str) {
    todo!()
}
fn run_bytecode(path: &str) {
    todo!()
}
