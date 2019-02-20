// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
  let name = "Brad";
  let mut age = 37;
  println!("My name is {} and I am {}", name, age);
  age = 38;
  println!("My name is {} and I am {}", name, age); // allowed -- mutable

  // Define constant
  const ID: i32 = 001; // you don't really use that much.. It is always immutable and you must assign a type to it
  println!("ID: {}", ID);

  // Assign multiple vars
  let ( my_name, my_age ) = ("Brad", 37); // tuple destructuring
  println!("{} is {}", my_name, my_age );
}
