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

fn main() {
    let flags: Flags = Flags::new();
    if flags.flag_error {}
}
