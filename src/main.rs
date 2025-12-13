mod encoder;
mod reader;
use std::path::PathBuf;
use clap::{CommandFactory, Error, Parser, Subcommand, error::ErrorKind};

#[derive(Parser)]
#[command(author, version, about, long_about= None)]
struct Args {
    /// File path of the WAV file. WARNING: Runnning this command as-is will just print out all the normalized samples of your program. Not very useful.
    #[arg(value_name = "FILE")]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let samples = reader::read_and_normalize_wav(args.path).unwrap();

    println!("{:#?}", samples.samples)
}
