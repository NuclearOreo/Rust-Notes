extern crate rand;
use rand::Rng;

pub fn run() {
    // Random Number
    let random_number = rand::thread_rng().gen_range(1, 11);
    println!("{}", random_number);

    // Flip a coin
    let random_bool = rand::thread_rng().gen_weighted_bool(25);
    println!("Random Bool: {}", random_bool);
}
