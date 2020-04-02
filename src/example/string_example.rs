pub fn run() {
    let mut my_string = String::from("The coronvirus is destroying the world :(");

    println!("{}", my_string.len());

    println!("{}", my_string.is_empty());

    for token in my_string.split_whitespace() {
        println!("{}", token);
    }

    my_string.push_str(" OMG");

    println!("{}", my_string);

    {
        let my_string = String::from("This is a sentence");

        println!("{}", my_string.replace("sentence", "dog"));
    }

    {
        let my_string = String::from("Hello\nWorlds\n");

        for line in my_string.lines() {
            println!("{}", line);
        }
    }

    {
        let my_string = String::from("This+is+a+sentence");
        let tokens: Vec<&str> = my_string.split("+").collect();

        println!("{:?}", tokens);
    }

    {
        let my_string = String::from("      So much whitespace       ");
        println!("{}", my_string.trim());
    }

    {
        let my_string = String::from("Watching video on Youtube");

        match my_string.chars().nth(4) {
            Some(c) => println!("{}", c),
            None => println!("Not found"),
        }
    }
}
