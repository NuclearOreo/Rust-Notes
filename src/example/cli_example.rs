use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    for arguments in args.iter() {
        println!("{}", arguments);
    }
}
