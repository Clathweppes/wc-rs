use clap::Parser;
use wc_rs::{Args, Options};

fn main() {
    let args: Args = Args::parse();
    let options = Options::build(&args);

    for filename in args.get_files() {
        match wc_rs::count_file(filename, &options) {
            Ok(counts) => wc_rs::print_counts(filename, &counts, &options),
            Err(e) => eprintln!("Error reading {}: {}", filename, e),
        }
    }
}
