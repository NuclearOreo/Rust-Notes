mod some_functions {
    pub fn hello_world() {
        println!("Hello World");
    }

    pub fn adding(x: i32, y: i32) -> i32 {
        return x + y;
    }

    pub mod nested {
        pub fn subtraction(x: i32, y: i32) -> i32 {
            return x - y;
        }
    }
}

pub fn run() {
    some_functions::hello_world();

    let adding_val = some_functions::adding(12, 6);
    let sub_val = some_functions::nested::subtraction(54, 12);

    println!("{}, {}", adding_val, sub_val);
}
