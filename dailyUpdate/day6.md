Rust — Vec<T> (Vector)

Great. You've finished arrays and tuples, so now we move to one of the most useful Rust data structures: Vec<T>.

A vector is basically a growable array.


let mut numbers: Vec<i32> = Vec::new();


numbers.push(10);
numbers.push(20);
numbers.push(30);

output : [10, 20, 30]


let numbers: Vec<i32> = Vec::new();

numbers.push(10); // ❌


Rust provides the vec! macro:

let mut numbers = vec![10, 20, 30];




let mut fruits = vec!["Apple", "Banana"];

fruits.push("Mango");

println!("{:?}", fruits);




let mut numbers = vec![10, 20, 30];

let removed = numbers.pop();

println!("{:?}", numbers);
println!("{:?}", removed);








let numbers = vec![10, 20, 30, 40];

println!("{}", numbers[0]);
println!("{}", numbers[2]);



let numbers = vec![10, 20, 30];

println!("{}", numbers.len());