use std::io;

pub fn run() {
    let mut input = String::new();

    println!("Please Enter in an input:");

    match io::stdin().read_line(&mut input) {
        Ok(_) => println!("Here's your input: {}", input),
        Err(e) => println!("Something went wrong:  {}", e),
    }
}
