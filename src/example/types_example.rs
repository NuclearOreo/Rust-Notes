pub fn run() {
    // Boolean
    let flag: bool = true;
    // Character
    let letter: char = 'A';

    // Integer (Signed / Unsigned)
    let signed: i32 = -32;
    let unsigned: u32 = 32;

    // Example of type casting
    let casted = unsigned as i32;

    // Float
    let pi_ish: f32 = 3.14;

    // Array
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    // Vector mutable
    let mut vector: Vec<i32> = vec![1, 2, 3, 5];
    vector.push(243);

    // Primitive String Immutable
    let string: &str = "2324";

    // Non Primitive Type
    let mut string2: String = String::from("Hello");
    string2.push_str(" World!");

    // Tuples
    let tuple = (flag, letter, signed, &array, &string2, "Yes it's a tuple");

    // Printing Everything

    println!("{}", flag);
    println!("{}", letter);
    println!("{}", signed);
    println!("{}", unsigned);
    println!("{}", casted);
    println!("{}", pi_ish);
    println!("{:?}", &array);
    println!("{:?}", &vector);
    println!("{:?}", &string);
    println!("{:?}", &string2);
    println!("{:?}", &tuple);
}
