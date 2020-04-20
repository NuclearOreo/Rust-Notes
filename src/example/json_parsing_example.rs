extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    is_male: bool,
}

pub fn run() {
    let json_string = r#"
        {
            "name": "Ussama",
            "age": 25,
            "is_male": true
        }
    "#;

    let result1 = serde_json::from_str(json_string);
    let result2 = serde_json::from_str(json_string);

    if result1.is_ok() {
        let p: JsonValue = result1.unwrap();
        println!("The name is {}", p["name"].as_str().unwrap());
    } else {
        println!("Can't parse json");
    }

    if result2.is_ok() {
        let p: Person = result2.unwrap();
        println!("The name is {}", p.name);
    } else {
        println!("Can't parse json");
    }
}
