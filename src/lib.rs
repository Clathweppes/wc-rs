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

#[derive(Default)]
pub struct Counts {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
    pub chars: usize,
    pub max_line_length: usize,
}

impl Counts {
    pub fn add(&mut self, other: &Counts) {
        self.lines += other.lines;
        self.words += other.words;
        self.bytes += other.bytes;
        self.chars += other.chars;
        self.max_line_length = self.max_line_length.max(other.max_line_length);
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_read_file() {
        let filename = "test_file.txt";
        let content = "Hello, world!\nThis is a test file.";
        let mut file = File::create(filename).unwrap();
        write!(file, "{}", content).unwrap();

        let result = read_file(filename).unwrap();
        assert_eq!(result, content);

        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_count_file() {
        let content = "Hello, world!\nThis is a test file.";
        let filename = "test_count_file.txt";
        let mut file = File::create(filename).unwrap();
        write!(file, "{}", content).unwrap();

        let options = Options {
            count_lines: true,
            count_words: true,
            count_bytes: true,
            count_chars: true,
            count_max_line_length: true,
        };

        let counts = count_file(filename, &options).unwrap();
        assert_eq!(counts.lines, 2);
        assert_eq!(counts.words, 7);
        assert_eq!(counts.bytes, content.len());
        assert_eq!(counts.chars, content.chars().count());
        assert_eq!(counts.max_line_length, 20);

        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_counts_add() {
        let mut counts1 = Counts {
            lines: 2,
            words: 5,
            bytes: 30,
            chars: 30,
            max_line_length: 15,
        };
        let counts2 = Counts {
            lines: 3,
            words: 10,
            bytes: 50,
            chars: 50,
            max_line_length: 20,
        };

        counts1.add(&counts2);

        assert_eq!(counts1.lines, 5);
        assert_eq!(counts1.words, 15);
        assert_eq!(counts1.bytes, 80);
        assert_eq!(counts1.chars, 80);
        assert_eq!(counts1.max_line_length, 20);
    }
}
