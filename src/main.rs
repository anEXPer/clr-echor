fn main() {
    let mut args: Vec<String> = std::env::args()
        .filter(|arg| !arg.starts_with("-"))
        .collect();

    args.remove(0);

    println!("{}", args.join(" "));
}
