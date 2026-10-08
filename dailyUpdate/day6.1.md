Rust — String vs &str

Let's take this slowly because this topic leads directly into ownership, which is one of the most important parts of Rust.

let name: &str = "Ashish";

A &str is a string slice.

For simple fixed text, this is very common:

let name = "Ashish";
let os_name = "AshOS";

println!("{}", name);

let name = "Ashish";

// name.push_str(" Kapoor"); ❌


2. String

String is an owned, growable string.

let mut name = String::from("Ashish");


name.push_str(" Kapoor");

println!("{}", name);