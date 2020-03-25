/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (Numbers of bit taken in memory)
Float: f32, f64
Boolean: (bool)
Characters: (char)
Tuples
Arrays
*/

// Rust is a statically typed language, which means that it must know
// the types of all variables at compile time, however, the compiler can
// usually infer what type we want to use based on the value and how we use it.

pub fn run() {
    // Default is i32
    let x = 1;

    // Default is f64
    let y = 2.5;

    // Explicitly set type
    let z: i64 = 44544565345;

    //Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i128: {}", std::i128::MAX);

    // Boolean
    let is_active: bool = true;

    // Get Bool from expression
    let is_greater: bool = 10 < 5;
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, a1, face, is_greater))
}
