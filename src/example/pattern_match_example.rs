pub fn run() {
    matcher(12);
    matcher(1);
    matcher(2);
    matcher(10);
    matcher(11);
    matcher(100);
    matcher(50);
}

fn matcher(number: i32) {
    match number {
        1 => println!("This is a one"),                   // matches one
        2 => println!("This a two"),                      // matches two
        10 | 11 => println!("This is either a 10 or 11"), // matches either 10 or 11
        50..=100 => println!("It is within the range of 50 to 100"), // matches within the range of 50 to 100
        _ => println!("Never seen this number"), // If it does not match anything
    }
}
