use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,

    #[clap(parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();
    
    let file = File::open(&args.path).expect("could not open the file :(");
    let content = BufReader::new(file);
    for line in content.lines() {
	if let Ok(line) = line {
        	if line.contains(&args.pattern) {
            		println!("{}", line);
        	}
	}
    }
}
