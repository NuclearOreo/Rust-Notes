use std::fs::File;
use std::io::prelude::*;

pub fn run() {
    let mut file1 = File::open("./file.txt").expect("Can't find file");
    let mut file2 = File::create("test.txt").expect("Can't Create file");

    let mut content = String::new();

    file1.read_to_string(&mut content).expect("Can't read file");

    file2
        .write_all(b"Is this working")
        .expect("Can't write to file");

    println!("{}", content);
}
