extern crate regex;
use regex::Regex;

pub fn run() {
    let re = Regex::new(r"(\w{5})").unwrap();
    let text = "dcode";

    println!("Found Match: {}", re.is_match(text));

    match re.captures(text) {
        Some(caps) => println!("Found Match: {}", caps.get(0).unwrap().as_str()),
        None => println!("Could not find the match"),
    }
}
