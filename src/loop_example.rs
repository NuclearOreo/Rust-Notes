pub fn run() {
    // Normal Infinite Loop
    let mut count = 0;

    loop {
        if count == 5 {
            count += 1;
            continue;
        }
        println!("Infinite Loop: {}", count);
        count += 1;
        if count == 10 {
            break;
        }
    }

    // While Loop
    count = 0;

    while count <= 10 {
        println!("While Loop: {}", count);
        count += 1;
    }

    // For Loop by range
    for i in 1..50 {
        println!("For Loop: {}", i);
    }

    // For loop using .iter()
    let vector = vec!["One", "Two", "Three"];

    for element in vector.iter() {
        println!("{}", element);
    }

    // For loop using .enumerate()
    for (index, element) in vector.iter().enumerate() {
        println!("{}, {}", index, element);
    }

    // For loop by passing a reference
    for element in &vector {
        println!("{}", element);
    }
}
