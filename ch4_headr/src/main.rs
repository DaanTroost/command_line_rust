#![allow(non_snake_case)]

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use anyhow::Result;
use clap::{Arg, Command};

#[derive(Debug)]
struct Arguments {
    files: Vec<String>,
    bytes: Option<u64>,
    lines: u64,
}

fn get_args() -> Arguments {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("Daan Troost <d.q.troost@outlook.com>")
        .about("Rust version of 'head'")
        .arg(
            Arg::new("bytes")
                .value_name("BYTES")
                .short('c')
                .long("bytes")
                .value_parser(clap::value_parser!(u64).range(1..))
                .help("How many bytes you want to print. Optional.")
                .conflicts_with("lines"),
        )
        .arg(
            Arg::new("lines")
                .value_name("LINES")
                .short('n')
                .long("lines")
                .help("How many lines you want to print. Defaults to 10.")
                .value_parser(clap::value_parser!(u64).range(1..))
                .default_value("10"),
        )
        .arg(
            Arg::new("file")
                .value_name("PATH")
                .help("Path(s) to the target file(s).")
                .default_value("-")
                .num_args(0..),
        )
        .get_matches();

    Arguments {
        files: matches.get_many("file").unwrap().cloned().collect(),
        lines: matches.get_one("lines").cloned().unwrap(),
        bytes: matches.get_one("bytes").cloned(),
    }
}

fn main() {
    if let Err(e) = run(get_args()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Arguments) -> Result<()> {
    let num_files = args.files.len();

    for (file_num, filename) in args.files.iter().enumerate() {
        match openFile(&filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(mut file) => {
                if num_files > 1 {
                    println!("{}==> {filename} <==", if file_num > 0 { "\n" } else { "" });
                }
                if let Some(num_bytes) = args.bytes {
                    let mut buffer = vec![0; num_bytes as usize];
                    let bytes = file.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes]));
                } else {
                    let mut line = String::new();

                    for _ in 0..args.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }

                        print!("{line}");
                        line.clear();
                    }
                }
            }
        };
    }
    Ok(())
}

fn openFile(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
