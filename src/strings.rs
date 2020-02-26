// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let mut hello = String::from("Hello");

    // Get length
    println!("{}", hello.len());

    hello.push(' ');

    hello.push_str("Worlds");

    println!("{}", hello);

    println!("Capacity: {}", hello.capacity());

    println!("Is empty: {}", hello.is_empty());

    let word = "Worlds";
    println!("Contain '{}': {}", word, hello.contains(word));

    hello = hello.replace(word, "There");

    println!("{}", hello);

    for word in hello.split(' ') {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}
