use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::{thread, time};

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,

    #[clap(parse(from_os_str))]
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    
    let file = File::open(&args.path)
	.with_context(|| format!("could not open file `{}`", &args.path.display()))?;

    let progress_bar = indicatif::ProgressBar::new(100);
    for _ in 0..100 {
        thread::sleep(time::Duration::from_millis(10));
        progress_bar.inc(1);
    }
    progress_bar.finish_with_message("results: ");

    let content = BufReader::new(file);
    for line in content.lines() {
	if let Ok(line) = line {
        	if line.contains(&args.pattern) {
            		println!("{}", line);
        	}
	}
    }

    Ok(())
}
