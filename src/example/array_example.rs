pub fn run() {
    let array = [1, 2, 3, 4, 5];
    let strict_array: [i32; 2] = [6, 7];
    let hellos = ["hello"; 20];

    for e in array.iter() {
        println!("{}", e);
    }

    for (i, e) in strict_array.iter().enumerate() {
        println!("Index: {}, Element: {}", i, e);
    }

    for i in 0..hellos.len() {
        println!("{}", hellos[i]);
    }

    println!("{:?}", hellos);
}
