pub fn run() {
    // Print to console
    println!("Hello from print.rs file");

    // Printing Number
    println!("Number: {}", 1);

    // More placeholder
    println!("{} is {}", "Brad", "Old");

    // Positional Arguments
    println!("Pattern: {0}, {0}, {1}, {1}, {2}, {2}", 1, 2, 3);

    // Named Arguments
    println!(
        "{name} likes to play {game}",
        name = "John",
        game = "Baseball"
    );

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug
    println!("{:?}", (12, true, "Hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10)
}
