use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
#[command(
    name = "My RPN program",
    version = "1.0.0",
    author = "Hashimoto Yoshiki",
    about = "Super awesome sample RPN calculator"
)]
struct Opts {
    #[arg(short, long)]
    verbose: bool,
    #[arg(name = "FILE", index = 1)]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    let Some(path) = opts.formula_file else {
        println!("No file specified");
        return;
    };
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}
