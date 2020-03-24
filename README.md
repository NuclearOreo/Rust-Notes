# My Rust Notes

## Important Notes

All rust projects will run from a main function. Just like in C/C++ or Java. Variable in Rust is immutable by default to make them mutable you have use `mut` after the `let` when declaring a varible.

## Rust CLI

- `cargo init`: Intialize a Cargo project in side a floder
- `cargo new <Path> --bin`: Create new Cargo with a given path
- `cargo build`: Create build project from source
- `carog run`: Build and run project from source
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
fn main() {
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
	let tuple = (flag, letter, signed, array, &string2, "Yes it's a tuple");
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
  [source](https://github.com/NuclearOreo/Rust-Notes/blob/master/src/loop_example.rs)

```rust
fn main() {
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

## Enums / Constants / Tuples

Enums are simple in rust all you have to type is `enum` then name and the curly braces containing the enumerables

- Enum

```rust
enum Direction {
	Up,
	Down,
	Left,
	Right
}
```

Constant are also simple in rust all you have to do is type `const` followed with a name and the specify the data type

- Constant

```rust
const NUMBER: u8 = 3;
```

Tuple are easy to set in rust. All you need is paratheses and any mixture of variables or data types in the in between it. To access the elements in the in the tuples you use the dot operator.

- Tuple

```rust
let tuple = (1, 2, 3, "one", 'c', [2, 3, 5], ('x', 'y'));

tuple.0; // 1
tuple.3; // "one"
tuple.5; // [2, 3, 5]
(tuple.6).0; // 'x'

// Unpacking Tuple

let (a, b, c, d, e, f, g) = (1, 2, 3, "one", 'c', [2, 3, 5], ('x', 'y'));
```

## Function / Code Block

To create function in rust you have use the keywork `fn` to create. You can use the the keywork `pub` before the function declaration to it public.

Code blocks are just blocks of code seperated by curly braces that have access to both inside and outside the scope of the block. It isolate itself in a similiar fashion of a function.

- Function

```rust
fn main() {
	println!("{}", is_even(30));
}

fn is_even(num: i32) -> bool {
	return num % 2 == 0;
}
```

- Code Block

```rust
fn main() {
	let x = 0;
	{
		let y = 10;
		println!("Printing x and y: {}, {}", x, y);
	}
	println!("Can only print x: {}", x);
}
```

[Example Source](<[https://github.com/NuclearOreo/Rust-Notes/blob/master/src/function_example.rs](https://github.com/NuclearOreo/Rust-Notes/blob/master/src/function_example.rs)>)

## References

You can pass references to variables in rust using an `&`, same as java and C/C++.

```rust
let mut x = 1234;
let rx = &x;

println!("{}", rx); // Will give you 1234
```

Things gets interesting when making a mutable reference in rust. When modifying a mutable reference in rust you need to create code to make change and then access the variable outside the codeblock. Also, you have to the `mut` keyword after the `&` to create a mutable reference and used the `*` keyword to create the change.

```rust
let mut x = 1234;
{
	let rx = &mut x;
	*x += 1;
}
println!("{}", x);
```

Also, can pass reference of variables to functions

```rust
fn main() {
	let mut array: [i32, 5] = [3, 5, 4, 6];
	sort(&mut array);
	println!("Yeah it's sorted: {:?}", array);
}

fn sort(nums: &mut [i32]) {
	for i in 0..nums.len() {
		for j in i..nums.len() {
			nums.swap(i, j);
		}
	}
}
```

## Structs

To create struct, you use the `struct` keywork to create it. There's two way to create struct in rust, a regular and tuple stuck.

- Regular Struct

```rust
struct Color {
	red: u8,
	green: u8,
	blue: u8
}

let bg = Color(red: 255, green: 0, blue: 0);
println!("{}, {}, {}", bg.red, bg.green, bg.blue);
```

- Tuple Struct

```rust
struct Color(u8, u8, u8);

let red = Color(255, 0, 0);
println!("{}, {}, {}", red.0, red.1, red.2);
```

## Arrays

Array are simple enough to use in rust. All have to use the is the normal square brackets to declare and then you iterator through them however you like.

- Declaration Example

```rust
let arr1 = [1, 2, 3, 4, 5]; // Implicitly Typed
let arr2: [i32; 2] = [6, 7]; // Explicitly Typed
let arr3 = ["hello"; 4]; // 4 "hellos" in a array
```

- Iteration Example

```rust
for e in arr1.iter() {
	println!("{}", e);
}

for (i, e) in arr2.iter().enumerate() {
	println!("Index: {}, Element: {}", i, e);
}

for i in 0..arr3.len() {
	println!("{}", arr3[i]);
}
```

[Example Source](https://github.com/NuclearOreo/Rust-Notes/blob/master/src/array_example.rs)

## Impl Keyword

`impl` keyword is similiar to the `class` keyword in other language except that in rust used a struct as the contastructor.

```rust
struct Rectangle {
	width: u32,
	height: u32
}

impl Retangle {
	fn is_square(&self) -> {
		return self.width == self.height;
	}
}
```

[Example Source](<[https://github.com/NuclearOreo/Rust-Notes/blob/master/src/impl_example.rs](https://github.com/NuclearOreo/Rust-Notes/blob/master/src/impl_example.rs)>)

## Strings

There's two types of string in rust. The classic primitive string and non-primitive string. The example below will be using the non-primitive since the other string isn't useful.

```rust
let  mut my_string =  String::from("The coronvirus is destroying the world :(");

println!("{}", my_string.len());

println!("{}", my_string.is_empty());

for token in my_string.split_whitespace() {
println!("{}", token);
}

my_string.push_str(" OMG");

println!("{}", my_string);
```

[Example Source ](<[https://github.com/NuclearOreo/Rust-Notes/blob/master/src/string_example.rs](https://github.com/NuclearOreo/Rust-Notes/blob/master/src/string_example.rs)>)  
[Documetation](<[https://doc.rust-lang.org/std/string/struct.String.html](https://doc.rust-lang.org/std/string/struct.String.html)>)
