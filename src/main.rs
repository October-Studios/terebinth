//     terebinth - lightweight interpreted programming language
//     Copyright (C) 2024 Cameron Howell
//
//     Licensed under the MIT License

use clap::Parser;
use log::{debug, info};
use std::error::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Terebinth source files
    #[arg(short, long)]
    in_files: Vec<String>,
    /// Operate in debug mode
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    if args.debug {
        debug!("Enabled debug-level logs");
    }

    for _ in 0..args.in_files.len() {
        info!("Reading in input files...")
    }

    // Create terebinth program object to interpret and execute
    // TerebinthProgram program;

    Ok(())
}
