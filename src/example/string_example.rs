pub fn run() {
    let mut my_string = String::from("The coronvirus is destroying the world :(");

    println!("{}", my_string.len());

    println!("{}", my_string.is_empty());

    for token in my_string.split_whitespace() {
        println!("{}", token);
    }

    my_string.push_str(" OMG");

    println!("{}", my_string);
}
