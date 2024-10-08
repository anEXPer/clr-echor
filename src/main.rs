#![warn(clippy::all, clippy::pedantic)]
use clap::Parser;

#[derive(Debug, Parser)]
/// A Rust version of `echo` written as practice
struct Args {
    ///Input text
    #[arg(required(true))]
    text: Vec<String>,

    ///Do not print newline
    #[arg(short('n'))]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();

    if args.omit_newline {
        print!("{}", args.text.join(" "));
    } else {
        println!("{}", args.text.join(" "));
    }
}
