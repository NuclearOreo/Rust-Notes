pub fn run () {
    let mut count = 0;

    // Infinite Loop
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 20 {
            break;
        }
    }

    // While Loop
    while count <= 100 {
        if count % 3 == 0 && count % 5 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        }
        count += 1; 
    }

    // For range loop
    for i in 0..100 {
        println!("{}", i);
    }
}