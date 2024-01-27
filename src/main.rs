#![warn(clippy::all, clippy::pedantic)]
use clap::Parser;

#[derive(Debug, Parser)]
/// Rust version `echo` written as practice
struct Args {
    ///Input text
    #[arg(required(true))]
    text: Vec<String>,

    ///Do not print newline
    #[arg(short('n'))]
    omit_newline: bool,

    ///Backwards compatability for ignore flags test
    #[arg(long("flag"))]
    ignore_flags: bool,
}

fn main() {
    let args = Args::parse();

    if args.omit_newline {
        print!("{}", args.text.join(" "));
    } else {
        println!("{}", args.text.join(" "));
    }
}
