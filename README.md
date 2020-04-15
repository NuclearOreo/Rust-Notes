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

- [Source](https://github.com/NuclearOreo/Rust-Notes/blob/master/src/example/types_example.rs)

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
  [source](https://github.com/NuclearOreo/Rust-Notes/blob/master/src/example/loop_example.rs)

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

[Example Source](https://github.com/NuclearOreo/Rust-Notes/blob/master/src/example/function_example.rs)

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

[Example Source](https://github.com/NuclearOreo/Rust-Notes/blob/master/src/example/array_example.rs)

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

[Example Source](https://github.com/NuclearOreo/Rust-Notes/blob/master/src/example/impl_example.rs)

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

- Replace

```rust
{
	let my_string =  String::from("This is a sentence");
	println!("{}", my_string.replace("sentence", "dog"));
}
```

- Lines

```rust
{

	let my_string =  String::from("Hello\nWorlds\n");
	for line in my_string.lines() {
		println!("{}", line);
	}
}
```

- Split

```rust
{
	let my_string =  String::from("This+is+a+sentence");
	let tokens: Vec<&str>  = my_string.split("+").collect();
	println!("{:?}", tokens);
}
```

- Trim

```rust
{
	let my_string =  String::from(" So much whitespace ");
	println!("{}", my_string.trim());
}
```

- Chars

```rust
{
	let my_string =  String::from("Watching video on Youtube");

	match my_string.chars().nth(4) {
		Some(c) => println!("{}", c),
		None => println!("Not found"),
	}
}
```

[Example Source ](https://github.com/NuclearOreo/Rust-Notes/blob/master/src/example/string_example.rs) <br >
[Documetation](https://doc.rust-lang.org/std/string/struct.String.html)

## Triats

Example below is showing how to use the ToString trait for a struct

```rust
struct  Person {
	name: String,
	age: u8,
}

impl ToString for  Person {
	fn  to_string(&self) -> String {
		return  format!("My name is {} and my age is {}", self.name, self.age);
	}
}
```

[Documentation](https://doc.rust-lang.org/rust-by-example/trait.html)

## Vector

Vectors growable arrays in rust, just like in C++. So push, pop and dynamically change an array how every you like. Way more useful than primitive arrays.

- Declaration

```rust
let vector_1 = Vec![1, 2, 3, 5];
let vector_2: Vec<i32> = vec![1, 3, 5, 6];
```

- What you can do

```rust
my_vec.pop();
my_vec.push(7);
my_vec.repeat(4);
my_vec.resize(3, 0);
```

[Documentation](https://doc.rust-lang.org/std/vec/struct.Vec.html)

## Read/Write Files

Reading and writing to and from files in rust is simple using the standard library. You just need `std::fs::File` and `std::io::prelude::*` to get started.

- Here an example below

```rust
use std::fs::File;
use std::io::prelude::*;

pub  fn  run() {
	let  mut file1 = File::open("./file.txt").expect("Can't find file");
	let  mut file2 = File::create("test.txt").expect("Can't Create file");

	let  mut content =  String::new();

	file1.read_to_string(&mut content).expect("Can't read file")

	file2
	.write_all(b"Is this working")
	.expect("Can't write to file");

	println!("{}", content);
	}
}
```

[Documention](https://doc.rust-lang.org/book/ch12-02-reading-a-file.html)

## Command Line Argument

Command line arguments are simple implement in rust using the standard library. All you need to use is `std::env` to get started.

- Example code

```rust
use std::env;

pub  fn  main() {
	let args: Vec<String>  = env::args().collect();
	for arguments in args.iter() {
	println!("{}", arguments);
	}
}
```

- Example Command
  `./binary arg1 arg2 arg3`

[Documentation](https://doc.rust-lang.org/std/env/index.html)

## Pattern Match

The `match` keyword is similar to the `switch` keyword in other languages. Given an input it will run through all the statement untill is matches one of them, normally running down the statement in order that they are written.

- Example Below

```rust
let number = 2;
match number {
	1 => println!("This is a one"), // matches one
	2 => println!("This a two"), // matches two
	10  |  11 => println!("This is either a 10 or 11"), // matches either 10 or 11
	50..=100 => println!("It is within the range of 50 to 100"), // matches within the range of 50 to 100
	_ => println!("Never seen this number"), // If it does not match anything
}
```

The example showcased here is is showing how `match` is setup. The `number` variable will be matched against the statements. The first two statements will match to a specific number. The third will match to two number, 10 and 11 to specifc. The fourth statement will mactch to a range of numbers, 50 to 100. And the last statement is a catch all, will match to anything.

## Reading Input

Reading user input rust is easy enough with the standard library that is it has. All you to do is import `std::io` and you should be ready to go.

- Example

```rust
use std::io;

fn  main() {
	let  mut input =  String::new();

	println!("Please Enter in an input:");

	match io::stdin().read_line(&mut input) {
		Ok(_) => println!("Here's your input: {}", input),
		Err(e) => println!("Something went wrong: {}", e),
	}
}
```

[Documentation](https://doc.rust-lang.org/std/io/struct.Stdin.html)

## HashMap

No language is really complete with the ultimate data structure, the hashmap :). It's simple enough to use in rust all you need import it from std/collections to start using it.

- Example

```rust
use std::collections::HashMap;

fn  main() {
	let  mut hashmap = HashMap::new();

	// Inserting into hashmap
	hashmap.insert("Rust Programming", 90);
	hashmap.insert("Web Dev", 91);
	hashmap.insert("UX/UI Design", 50);

	// Length of Hashmap
	println!("Number of Courses: {}", hashmap.len());

	// Getting a Value using Key
	match hashmap.get("Web Dev") {
		Some(val) => println!("Score: {}", val),
		None => println!("Didn't take the course"),
	}

	// Removing an Key Value pair
	hashmap.remove("UX/UI Design");

	// Looping through all the Key Value pairs
	for (subject, score) in &hashmap {
		println!("Subject: {}, Score: {}", subject, score);
	}

	// Checking if to contains a specific key
	println!("Did you take C++?: {}", hashmap.contains_key("C++"));
}
```

[Documentation](https://doc.rust-lang.org/nightly/std/collections/struct.HashMap.html)

## Random

Generating random is possible in rust but not of the standard library. To create random numbers you have to get an external crate `rand` to get it.

- Example `Cargo.toml`

```
[package]
name = "rust"
version = "0.1.0"
authors = ["John Doe <johnDoe@gmail.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[dependencies]
rand = "0.3"
```

- Example

```rust
extern  crate rand;
use rand::Rng;

fn  main() {
	// Random Number
	let random_number = rand::thread_rng().gen_range(1, 11);
	println!("{}", random_number);

	// Flip a coin
	let random_bool = rand::thread_rng().gen_weighted_bool(25);
	println!("Random Bool: {}", random_bool);
}
```

[Source](https://github.com/NuclearOreo/Rust-Notes/blob/master/src/example/random_number_example.rs)

## Regular Experssion

Using regular experssion in Rust you have to the same as when using random. Adding it as dependancy in the `Cargo.toml`. Once you have it there you should be ready to go.

- Example `Cargo.toml`

```
[package]
name = "rust"
version = "0.1.0"
authors = ["John Doe <johnDoe@gmail.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[dependencies]
regrex = "0.2"
```

Below is an example code of how to use regrex in rust. Similiar to most langauge and very similiar to python.

- Example Code

```rust
extern  crate regex;
use regex::Regex;

fn  main() {
	let re = Regex::new(r"(\w{5})").unwrap();
	let text =  "dcode";

	println!("Found Match: {}", re.is_match(text));

	match re.captures(text) {
	Some(caps) => println!("Found Match: {}", caps.get(0).unwrap().as_str()),
		None => println!("Could not find the match"),
	}
}
```

**Note:** To use `captures` you need to wrap your regular expression with paratheses or else it won't work.

[Documentation](https://docs.rs/regex/1.3.6/regex/)

## Modules

Modules are similiar to namespaces in C++. A way to group a butch of functions together by using a keyword `mod`. Pretty simple and straight forward to use.

```rust
mod some_functions {

	pub  fn  hello_world() {
		println!("Hello World");
	}

	pub  fn  adding(x: i32, y: i32) -> i32 {
		return x + y;
	}

	pub  mod nested {
		pub  fn  subtraction(x: i32, y: i32) -> i32 {
			return x - y;
		}
	}
}

fn  main() {

	some_functions::hello_world();

	let adding_val = some_functions::adding(12, 6);
	let sub_val = some_functions::nested::subtraction(54, 12);

	println!("{}, {}", adding_val, sub_val);
}
```

[Documentation](https://doc.rust-lang.org/rust-by-example/mod.html)
