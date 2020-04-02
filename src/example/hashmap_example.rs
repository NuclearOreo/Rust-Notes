use std::collections::HashMap;

pub fn run() {
    let mut hashmap = HashMap::new();

    // Inserting  into hashmap
    hashmap.insert("Rust Programming", 90);
    hashmap.insert("Web Dev", 91);
    hashmap.insert("UX/UI Design", 50);

    // Length of Hashmap
    println!("Number of Courses: {}", hashmap.len());

    match hashmap.get("Web Dev") {
        Some(val) => println!("Score: {}", val),
        None => println!("Didn't take the course"),
    }

    hashmap.remove("UX/UI Design");

    for (subject, score) in &hashmap {
        println!("Subject: {}, Score: {}", subject, score);
    }

    println!("Did you take C++?: {}", hashmap.contains_key("C++"));
}
