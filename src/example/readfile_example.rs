use std::fs::File;
use std::io::prelude::*;

pub fn run() {
    let mut file = File::open("./file.txt").expect("Can't find file");

    let mut content = String::new();

    file.read_to_string(&mut content).expect("Can't read file");

    println!("{}", content);
}
