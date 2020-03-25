struct Person {
    name: String,
    age: u8,
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("My name is {} and my age is {}", self.name, self.age);
    }
}

pub fn run() {
    let person: Person = Person {
        name: String::from("Ussama Azam"),
        age: 24,
    };

    println!("{}", person.to_string());
}
