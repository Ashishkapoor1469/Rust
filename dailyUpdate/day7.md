Rust — Ownership
This is one of the most important Rust concepts. Take it slowly.

You predicted "ok", so now let's see what actually happens.

fn main() {
    let name = String::from("Ashish");

    let name2 = name;

    println!("{}", name);
}

output error[E0382]: borrow of moved value: `name`

 What is ownership?
 Every piece of data in Rust has an owner.

let name = String::from("Ashish");
 name
 │
 ▼
┌─────────────┐
│   Ashish    │
└─────────────┘

let name2 = name;


name ────────┐
             ├──> "Ashish"
name2 ───────┘

Before:

name
 ↓
"Ashish"


After:

name2
 ↓
"Ashish"

name → ❌ no longer owns it


println!("{}", name); //invalid


String
   │
   ├── let b = a;
   │       ↓
   │      MOVE
   │
   └── let b = a.clone();
           ↓
          COPY