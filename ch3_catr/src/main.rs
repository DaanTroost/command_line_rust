

#[derive(Debug)]
struct Args {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

fn get_args() -> Args {
    let matches = Command::new("catr")
    .version("0.1.0")
    .author("Daniël Quirinus Troost <d.q.troost@outlook.com")
    .about("cat, written in Rust btw")

    .get_matches();

    Args {
        files: ...
        number_lines: ...
        number_nonblank_lines: ...
    }
}

fn main() {
    
    let args = get_args();
    println!("{args:#?}");
}
