use clap::Parser;
use anyhow::Result;


#[derive(Debug, Parser)]
#[command(author = "Daniël Quirinus Troost<d.q.troost@outlook.com>")]
#[command(version = "0.1.0")]
#[command(
    help_template = "Author: {author-with-newline} {about-section} \n Version: {version} \n\n {usage-heading} \n {usage} \n\n{all-args}"
)]
#[command(about)]
///'cat', written in Rust, btw
struct Arguments {
    ///File to be printed to stdout
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    ///Set this flag to number the lines, including blank lines.
    #[arg(
        short('n'),
        long("number-lines"),
        conflicts_with("number_nonblank_lines")
    )]
    number_lines: bool,

    ///Set this flag to number the lines, skipping blank lines.
    #[arg(short('b'), long("number-nonblank-lines"))]
    number_nonblank_lines: bool,
}

fn main() {
    if let Err(e) = run(Arguments::parse()) {
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
