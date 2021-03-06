fn icon_types() -> String {
    "Icon types: battery, wifi, volume, calendar, clock".to_string()
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
        "wifi" => wifi(),
        "volume" => volume(&args),
        "calendar" => calendar(),
        "clock" => clock(),
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

    let state = args[2].to_lowercase();
    let level:u32 = args[3].parse().unwrap();

    if state == "charging" {
        return " ".to_string();
    } else if state == "full" {
        return "FULL".to_string();
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

fn wifi() -> String {
    " ".to_string()
}

fn volume_usage(args: &Vec<String>) -> String {
    format!("Usage: {} {} muted percentage", args[0], args[1])
}

fn volume(args: &Vec<String>) -> String {
    if args.len() < 2 {
        return volume_usage(args);
    }

    let muted = args[2].to_lowercase();
    let level:u32 = args[3].parse().unwrap();

    if muted == "muted" {
        return " ".to_string();
    }

    if level == 0 {
        return " ".to_string();
    } else if level < 50 {
        return " ".to_string();
    } else {
        return " ".to_string();
    }
}

fn calendar() -> String {
    " ".to_string()
}

fn clock() -> String {
    " ".to_string()
}
