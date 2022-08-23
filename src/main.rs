
use clap::Parser;
use grrs::find_matches;
use anyhow::{Context,Error};

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



fn main() -> Result<(), Error> {
    env_logger::init();
    info!("Starting up");

    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file {:?}", &args.path))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    info!("Shutting down");
    Ok(())
}
