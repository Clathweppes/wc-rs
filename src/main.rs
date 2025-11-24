use clap::Parser;
use wc_rs::{Args, Options};

fn main() {
    let args: Args = Args::parse();
    let options = Options::build(&args);
    let mut total_counts = wc_rs::Counts::default();
    let mut success_count = 0;

    for filename in args.get_files() {
        match wc_rs::count_file(filename, &options) {
            Ok(counts) => {
                wc_rs::print_counts(filename, &counts, &options);
                total_counts.add(&counts);
                success_count += 1;
            }
            Err(e) => {
                eprintln!("Error reading {}: {}", filename, e);
                std::process::exit(1);
            }
        }
    }

    if success_count > 1 {
        wc_rs::print_counts("total", &total_counts, &options);
    }
}
