use clap::Command;

#[derive(Debug)]
struct Args {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}


fn main() {
    println!("Hello, world!");
}

fn get_args() -> Args {
    let matches = Command::new("wcr")
        .version("0.1.0")
        .author("Daan Troost<d.q.troost@outlook.com")
        .about("Wicker, wc written in Rust btw")
        .arg(
            Arg::new("lines")
                .value_name("LINES")
                .short('l')
                .long("lines")
                .action(ArgAction::SetTrue)
                .help("Show how many lines there are in the file(s)")
        )
        .arg (
            Arg::new("words")
                .value_name("words")
                .short('w')
                .long("words")
                .action(ArgAction::SetTrue)
                .help("Show how many words there are in the given file(s).")
        )




    
}