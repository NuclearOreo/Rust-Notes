# My Rust Notes

## Important Notes

All rust projects will run from a main function. Just like in C/C++ or Java. Variable in Rust is immutable by default to make them mutable you have use `mut` after the `let` when declaring a varible.

## Rust CLI

- `cargo init`: Intialize a Cargo project in side a floder
- `cargo new <Path> --bin`: Create new Cargo with a given path
- `cargo build`: Create build project from source
- `cargo run`: Build and run project from source
- `cargo build --release`: Build project for project
- `rustc --version`: Returing the version of the rust complier installed
- `rustc <File Name>`: Compling a specific rust source file

## Comments

- One line comments are created using `\\`

**Example**

```rust
// Single Line comments
fn main() {
	println!("Hello World");
}
```

- Multi line comments are created using `/* Some comment */`

**Example**

```rust
/*
This
is
a
Multi
line
comment
*/
fn main() {
	println!("Yeah for comments");
}
```

## Variables / Data Types

- Signed/Unsigned Integer: `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `i64`, `u64`, `u128`, `i128` and more
- Floats: `f32`, `f64`
- Boolean: `bool`
- Character: `char`
- Array: `[i32; 5]` array with five 32-bit integers elements
- Vector: `Vec<i32>` growable array with 32-bit integers
- Primitive String: `&str`
- Non Primitive String: `String`
- Tuple is a mix of types in between parentheses

### Example

- [Source](https://github.com/NuclearOreo/Rust-Notes/blob/master/src/types_example.rs)

```rust
pub fn main() {
	// Boolean
	let flag: bool  =  true;

	// Character
	let letter: char  =  'A';

	// Integer (Signed / Unsigned)
	let signed: i32  =  -32;
	let unsigned: u32  =  32;

	// Example of type casting
	let casted = unsigned as i32;

	// Float
	let pi_ish: f32  =  3.14;

	// Array
	let array: [i32; 5] = [1, 2, 3, 4, 5];

	// Vector mutable
	let  mut vector: Vec<i32>  =  vec![1, 2, 3, 5];
	vector.push(243);

	// Primitive String Immutable
	let string: &str  =  "2324";

	// Non Primitive Type
	let  mut string2: String  =  String::from("Hello");
	string2.push_str(" World!");

	// Tuples
	let tuple = (flag, letter, signed, &array, &string2, "Yes it's a tuple");
}
```

## Loops

There's three types of loop in rust. The first first type is a infinite loop, which takes no condition and will continue to loop until you break out of it. The second type of loop is a while loop, which takes a condition and will end once the condition evaluate to false. The third type of loop is a for loop, while take an iterator and runs through each item in the iterator.

- Infinite Loop

```rust
let mut count =  0;
loop {
	println!("{}", count);
	count +=  1;
	if count ==  10 {
		break;
	}
}
```

- While Loop

```rust
let mut count = 0;
while count <= 10 {
	println!("{}", count);
	count += 1;
}
```

- For Loop

```rust
for i in 1..10 {
	println!("{}", i);
}
```

**Note**: When using for loop on vectors or arrays make sure to use the `.iter()` for vectors or arrays so they don't move out of place.

- Example

```rust
pub fn main() {
	// Normal Infinite Loop
	let mut count = 0;
	loop {
		if count == 5 {
			count += 1;
			continue;
		}
		println!("Infinite Loop: {}", count);
		count +=  1;
		if count ==  10 {
			break;
		}
	}

	// While Loop
	count =  0;
	while count <=  10 {
		println!("While Loop: {}", count);
		count +=  1;
	}

	// For Loop by range
	for i in  1..50 {
		println!("For Loop: {}", i);
	}

	// For loop using .iter()
	let vector =  vec!["One", "Two", "Three"];
	for element in vector.iter() {
		println!("{}", element);
	}

	// For loop using .enumerate()
	for (index, element) in vector.iter().enumerate() {
		println!("{}, {}", index, element);
	}

	// For loop by passing a reference
	for element in &vector {
		println!("{}", element);
	}
}
```
