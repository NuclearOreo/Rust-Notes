// Tuples group together values of different types
// Max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Ussama", "Miami", 25);

    println!("{:?}", person);
    println!("{} from {}, age {}", person.0, person.1, person.2);
}