use clap::Parser;
use std::io::Read;

#[derive(Parser)]
#[command(version, about = "Count lines, words, or bytes.", long_about = None)]
pub struct Args {
    #[arg(short = 'l', help = "Count lines")]
    lines: bool,

    #[arg(short = 'w', help = "Count words")]
    words: bool,

    #[arg(short = 'c', help = "Count bytes")]
    bytes: bool,

    #[arg(short = 'm', help = "Count characters")]
    chars: bool,

    #[arg(short = 'L', help = "Count maximum line length")]
    max_line_length: bool,

    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,
}

impl Args {
    pub fn get_files(&self) -> &Vec<String> {
        &self.files
    }
}

pub struct Options {
    count_lines: bool,
    count_words: bool,
    count_bytes: bool,
    count_chars: bool,
    count_max_line_length: bool,
}

impl Options {
    pub fn build(args: &Args) -> Self {
        let mut lines = args.lines;
        let mut words = args.words;
        let mut bytes = args.bytes;
        let chars = args.chars;
        let max_line_length = args.max_line_length;
        if !lines && !words && !bytes && !chars && !max_line_length {
            lines = true;
            words = true;
            bytes = true;
        }
        Options {
            count_lines: lines,
            count_words: words,
            count_bytes: bytes,
            count_chars: chars,
            count_max_line_length: max_line_length,
        }
    }
}

pub struct Counts {
    lines: usize,
    words: usize,
    bytes: usize,
    chars: usize,
    max_line_length: usize,
}

pub fn read_file(filename: &str) -> Result<String, std::io::Error> {
    if filename == "-" {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    } else {
        std::fs::read_to_string(filename)
    }
}

pub fn count_file(filename: &str, options: &Options) -> Result<Counts, std::io::Error> {
    let context = read_file(filename)?;
    let mut counts = Counts {
        lines: 0,
        words: 0,
        bytes: 0,
        chars: 0,
        max_line_length: 0,
    };
    if options.count_lines {
        counts.lines = context.lines().count();
    }
    if options.count_words {
        counts.words = context.split_whitespace().count();
    }
    if options.count_bytes {
        counts.bytes = context.len();
    }
    if options.count_chars {
        counts.chars = context.chars().count();
    }
    if options.count_max_line_length {
        counts.max_line_length = context.lines().map(|line| line.len()).max().unwrap_or(0);
    }

    Ok(counts)
}

pub fn print_counts(filename: &str, counts: &Counts, options: &Options) {
    if options.count_lines {
        print!("{:>8} ", counts.lines);
    }
    if options.count_words {
        print!("{:>8} ", counts.words);
    }
    if options.count_bytes {
        print!("{:>8} ", counts.bytes);
    }
    if options.count_chars {
        print!("{:>8} ", counts.chars);
    }
    if options.count_max_line_length {
        print!("{:>8} ", counts.max_line_length);
    }
    if filename != "-" {
        print!("{}", filename);
    }
    println!();
}
