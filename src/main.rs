fn icon_types() -> String {
    "Icon types: battery".to_string()
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} icon_type <args>", args[0]);
        return;
    }

    let icon_type = &args[1].to_lowercase();
    println!("{}", match icon_type.as_ref() {
        "battery" => battery(&args),
        _ => icon_types(),
    })
}

fn battery_usage(args: &Vec<String>) -> String {
    format!("Usage: {} {} state percentage", args[0], args[1])
}

fn battery(args: &Vec<String>) -> String {
    if args.len() < 4 {
        return battery_usage(args);
    }

    let state = &args[2].to_lowercase();
    let per_str = &args[3];
    let level:u32 = per_str.parse().unwrap();

    if state == "charging" {
        return " ".to_string();
    }

    //on battery
    if level < 20 {
        return " ".to_string();
    } else if level < 40 {
        return " ".to_string();
    } else if level < 60 {
        return " ".to_string();
    } else if level < 80 {
        return " ".to_string();
    } else {
        return " ".to_string();
    }
}
