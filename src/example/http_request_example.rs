extern crate reqwest;

pub fn run() {
    // Short way to make a request
    let response = reqwest::get("https://jsonplaceholder.typicode.com/todos/1")
        .expect("Failed Request")
        .text()
        .expect("Unable to read response");

    println!("{}", response);

    println!("\n\n");

    // Long way to make a request
    match reqwest::get("https://jsonplaceholder.typicode.com/users/1") {
        Ok(mut response) => {
            if response.status().is_success() {
                match response.text() {
                    Ok(text) => println!("{}", text),
                    Err(_) => println!("Unable to read text"),
                }
            } else {
                println!("Status Code wasn't 200")
            }
        }
        Err(_) => println!("Failed Response"),
    }
}
