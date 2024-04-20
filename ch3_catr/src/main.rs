use clap::{Arg, ArgAction, Command, Parser};
use anyhow::Result;


#[derive(Debug)]
struct Arguments {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

fn get_args() -> Arguments {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Daniël Quirinus Troost <d.q.troost@outlook.com")
        .about("cat, written in Rust btw")
        .arg(
            Arg::new("filename")
                .value_name("FILENAME")
                .help("File to be printed to stdout")
                .required(true)
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("linenumbers")
                .short('n')
                .long("show-linenumbers")
                .help("Set this flag to show line numbers, including blank lines")
                .action(ArgAction::SetTrue)
                .conflicts_with("non-blank_linenumbers"),
        )
        .arg(
            Arg::new("non-blank_linenumbers")
                .short('b')
                .long("show-nonblank-linenumbers")
                .help("Set this flag to show line numbers, skipping blank lines")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    Arguments {
        files: matches.get_many("filename").unwrap().cloned().collect(),
        number_lines: matches.get_flag("linenumbers"),
        number_nonblank_lines: matches.get_flag("non-blank_linenumbers"),
    }
}

fn main() {
    if let Err(e) = run(get_args()) {
        eprintln!("{e}");
        std::process::exit(1);
    }

}

fn run(args: Arguments) -> Result<()> {
    for filename in args.files {
        println!("{filename}");
    }
    Ok(())
}
