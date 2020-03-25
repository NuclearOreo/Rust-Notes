// Variables hold primitive to data or reference to data
// Variables are immutable by default
// Rust is block-scoped language

pub fn run() {
    let name = "Brad";
    let mut age = 37;

    println!("My name is {} and I'm {}", name, age);

    age = 38;

    println!("My name is {} and I'm {}", name, age);

    // Define Constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multple Variables
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}
