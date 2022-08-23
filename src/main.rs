use anyhow::{Context, Error};
use clap::Parser;
use grrs::find_matches;

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

    if args.pattern.is_empty(){
        return Err(Error::msg("An empty pattern is not allowed"));
    }

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file {:?}", &args.path))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    info!("Shutting down");
    Ok(())
}
