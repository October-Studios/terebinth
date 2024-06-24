//     terebinth - lightweight interpreted programming language
//     Copyright (C) 2024 Cameron Howell
//
//     Licensed under the MIT License

use log::{error, info};
use std::error::Error;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct Flags {
    pub path: String,
    pub in_files: Vec<String>,
    pub debug: bool,
    pub help: bool,
    pub version: bool,
    pub run_interpreted: bool,
    pub cpp_out_file: String,
    pub bin_out_file: String,
    pub run_compiled: bool,
    pub flag_error: bool,
}

impl Flags {
    #[must_use]
    pub fn new() -> Self {
        Flags {
            path: "".to_string(),
            in_files: vec![],
            debug: false,
            help: false,
            version: false,
            run_interpreted: true,
            cpp_out_file: "".to_string(),
            bin_out_file: "".to_string(),
            run_compiled: false,
            flag_error: false,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let flags: Flags = Flags::new();
    if flags.flag_error {
        error!("Try 'terebinth -h' for help");
        return Ok(());
    }

    if flags.help {
        println!("terebinth v {}", VERSION);
        println!();
        return Ok(());
    }

    if flags.version {
        info!("terebinth v {}", VERSION);
        return Ok(());
    }

    Ok(())
}
