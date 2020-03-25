// Arrays - Fixed list where elements are the same data types
use std::mem;

pub fn run() {
    let numbers: [i32; 5] = [1,2,3,4,5];

    println!("{:?}", numbers);

    // Grabing single element
    println!("First element: {}", numbers[0]);

    let mut numbers_mut: [i32; 5] = [1,2,3,4,5];

    numbers_mut[4] = 20;
    println!("Last element: {}", numbers_mut[4]);

    // Get Array Length
    println!("Get array length: {}", numbers_mut.len());

    // Array are stacked Allocated
    println!("Byte in Array: {}", mem::size_of_val(&numbers_mut));

    // Array Slicing
    let slice: &[i32] = &numbers_mut[1..3];

    println!("{:?}", slice);
}