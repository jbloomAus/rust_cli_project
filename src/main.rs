use clap::Parser;
use anyhow::{Context, Result};

#[macro_use]
extern crate log;

#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}


fn find_matches(content: &str, pattern: &str){
    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();
    info!("Starting up");

    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file {:?}", &args.path))?;

    find_matches(&content, &args.pattern);

    info!("Shutting down");
    Ok(())
}
