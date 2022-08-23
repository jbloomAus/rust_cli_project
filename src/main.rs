use clap::Parser;

#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}


fn main() {
    let args = Cli::parse();
    
    println!("Hello, world!");
    println!("Searching for {}", args.pattern);
    println!("In file {}", args.path.display());
}
