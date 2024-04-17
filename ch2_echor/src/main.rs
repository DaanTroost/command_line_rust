use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
//Echo, written in rust btw
struct Args {
    // text to be echoed
    #[arg(required(true))]
    text: Vec<String>,

    //omit ending newline
    #[arg(short('n'))]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();
    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline { "" } else { "\n" }
    );
}