use anyhow::Result;
use clap::Parser;
use std::fs::File;
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
            Ok(file) => {
                let mut blank_lines = 0;

                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;

                    if args.number_lines {
                        println!("{:>6}\t{line}", line_num + 1);
                    } else if args.number_nonblank_lines {
                    
                        if line.is_empty() {
                            println!();
                            blank_lines += 1;
                        } else {
                            println!("{:>6}\t{line}", line_num - blank_lines + 1)
                        }
                    
                    } else {
                        println!("{line}")
                    }
                }
            }
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
