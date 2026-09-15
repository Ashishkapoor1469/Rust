// fn main() {
//--------------Variables------------------//
//  let age = 25;
//  println!("Age: {}",age)

// let age = 25;   //not work because in rust all variable is intialy imutable
// age = 42;
// println!("Age: {}",age)

//----mutable type-------//

// let mut age:i32 = 34;  //by useing mut keyword we can change variable to mutable  can be changed later
// println!(" before Age: {}",age);
// age = 45;
// println!("after Age: {}",age);
//  const MAX_P:u32 = 29;   // u32 is a type
//  println!("Max val: {}",MAX_P)

//-------------variable size-------------//

// let username:&str = "Ashish";   // &str type of string
// let age:i32 = 22;  //i32 type of int

// let mut score:i32 =0; use mut when variable value changes again and again
// score +=10;

//===========const===============//

// mainly use like place
// const MAX_MEMORY:u64 = 4096;
// const VERSION: &str = "0.0.1";

// const PAGE_SIZE:usize= 4096;

//===========Shadowing=========//

// let age = 24;  // declare same vaiable name two or more time called shadowing
// let age = age +1;  // it is different from mut(chaneges existing variable)

// println!("Age :{}",age)    // but in shadowing a new binding that shadows the previous one

//==========DATA TYPE============//

// let temp: i32 = -10;  //signed
// let age :u32 = 24;// unsigned

//=========Float==========//

// let height :f32=5.7; // less precision
// let pi:f64=3.1415926535; //high precision

//=======Boolean=========//
// let is_running:bool = true;

// if is_running{
//     println!("Running")
// }

//==========char=============

// let letter: char = 'A';
// let symbol: char = '📍';
// println!("{}{}",letter,symbol)

// 'A'     → char
// "A"     → string slice

//==========String========//

// let name : &str = "Ashish"; // A &str is a borrowed string slice.

// let mut name:String = String::from("Ash");  //A String is an owned, growable string:

// name.push_str(  "OS");

// println!("{}",name)

//==========Function============//
//     greet();

//     greet_name("Ashish\n");

//   let sum:i32 = add(3, 4);
//   println!("{}",sum);

//============ Mini exercise ===========//

// println!("=====================");
// println!("       Student");
// println!("=====================");

// let name :&str = "Ashish";
// let age: i32 = 22;
// let course:&str = "BCA";

// let mut college:String = String::from("Dronacharya");
// college.push_str("College");
// println!("Name: {}",name);
// println!("Age: {}",age);
// println!("Course: {}",course);
// println!("College: {}",college);

//     greet_name("Ashish\n");
//  let agenextyear :i32 = calculate_age_next_year(age);
//  println!("Next year you will be {}",agenextyear)

// }

// fn greet() {
//     println!("Helo world")
// }
// fn greet_name(name: &str) {
//     print!("Heloo, {}", name)
// }

// fn add(a: i32, b: i32)->i32 {
//     a + b //also wirte  return a+b
// }

// fn calculate_age_next_year(age:i32)->i32{
//    age+1
// }

fn main() {
    // let age =22;
    // if age>=18{
    //     println!("Adult");
    // }else {
    //     println!("Minor");
    // }

    let age = 22;
    let status = if age >= 18 { "Adult" } else { "Minor" };
    println!("{}",status);
}
