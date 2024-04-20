use anyhow::Result;
use clap::Parser;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};

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
        match open_file(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(_) => read_file(&filename, &args.number_lines, &args.number_nonblank_lines)?,
        }
    }
    Ok(())
}

fn open_file(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn read_file(filename: &str, flag_number: &bool, flag_number_nonblank: &bool) -> Result<()> {
    let content = fs::read_to_string(filename)?;

    let mut split_lines: Vec<String> = vec![];

    for (index, line) in content.lines().enumerate() {
        let index_string: String = index.to_string();
        let mut new_line = line.to_string();
        if *flag_number {
            new_line = index_string + " " + line;
        }
        
        split_lines.push(new_line);
    }

    for line in split_lines {
        println!("{line}");
    }

    Ok(())
}
