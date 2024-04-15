use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Daniël Quirinus Troost <d.q.troost@outlook.com")
        .about("cmd 'echo', written in Rust btw")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_ending_newline")
                .short('n')
                .long("No-Newline")
                .action(ArgAction::SetTrue)
                .help("If set, echor will omit a closing newline at the end of the text."),
        )
        .get_matches();

    let text: Vec<String> = matches.get_many("text").unwrap().cloned().collect();

    let omit_newline = matches.get_flag("omit_ending_newline");
    let ending = if omit_newline { "" } else { "\n" };

    print!("{:#?}{ending}", text.join(" "));
}
