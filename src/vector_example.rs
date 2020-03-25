pub fn run() {
    let mut my_vec: Vec<i32> = Vec::new();

    println!("{:?}", my_vec);

    my_vec = vec![1, 2, 3, 5];

    println!("{:?}", my_vec);

    my_vec.pop();

    my_vec.push(7);

    my_vec.repeat(4);

    my_vec.resize(3, 0);
}
