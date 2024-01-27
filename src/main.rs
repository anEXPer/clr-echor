#![warn(clippy::all, clippy::pedantic)]

fn main() {
    let mut args: Vec<String> = std::env::args()
        .filter(|arg| !arg.starts_with('-'))
        .collect();

    args.remove(0);

    if std::env::args().len() > args.len() + 1 {
        print!("{}", args.join(" "));
    } else {
        println!("{}", args.join(" "));
    }
}
