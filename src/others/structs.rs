// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct TupleColor(u8, u8, u8);

// Using Functions in struct
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        blue: 0,
        green: 0,
    };

    c.green = 50;

    println!("Color: {}, {}, {}", c.red, c.green, c.blue);

    let tc = TupleColor(255, 0, 1);

    println!("Tuple Color: {}, {}, {}", tc.0, tc.1, tc.2);

    let mut p = Person::new("Mary", "Doe");
    println!("Person {}", p.full_name());
    p.set_last_name("Williams");
    println!("Person {}", p.full_name());
    println!("Person Tuple {:?}", p.to_tuple());
}
