fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} icon_type percentage", args[0]);
        return;
    }
}

