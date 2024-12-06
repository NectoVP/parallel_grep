use pargrep::run;
use pargrep::Event;
use core::fmt;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use clap::Parser;

struct MyWrapperMatch(pargrep::Match);

impl fmt::Display for MyWrapperMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "line_number: {}", self.0.line_number)?;
        writeln!(f, "line: {}", self.0.line)?;
        writeln!(f, "file: {}", self.0.path.to_string_lossy())?;
        Ok(())
    }
}

struct MyWrapperError(pargrep::Error);

impl fmt::Display for MyWrapperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "error: {}", self.0.error)?;
        writeln!(f, "file: {}", self.0.path.to_string_lossy())?;
        Ok(())
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'p', long = "pattern")]
    pattern: String,

    #[arg(short = 'm', long = "max-depth", default_value_t = -1)]
    max_depth: i32,

    #[arg(short = 'd', long = "directory")]
    directory: String,

    #[arg(short = 'f', long = "file_name", default_value = r"\.")]
    file_name: String,
}

fn main() {
    let args = Args::parse();
    let counter = AtomicUsize::new(0);
    for elem in run(args.directory, &args.pattern, args.max_depth, 0, &args.file_name,  &counter) {
        match elem {
            Event::Match(res) => println!("{}", MyWrapperMatch(res)),
            Event::Error(error) => println!("{}\n", MyWrapperError(error)),
        }
    }
    println!("files looked into: {}", counter.load(Ordering::SeqCst));
}
