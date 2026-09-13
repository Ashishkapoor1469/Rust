fn main() {
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



}