pub fn run() {
    println!(
        "Have you met Johnny: {}",
        match match_you_met("Johnny") {
            Some(o) => o.to_string(),
            None => "Never seen this name".to_string(),
        }
    );
}

fn match_you_met(name: &str) -> Option<bool> {
    match name {
        "Bob" => Some(true),
        "John" => Some(false),
        "Lilly" => Some(true),
        _ => None,
    }
}
