// Vectors - Resizable arrays

use std::mem;

pub fn run() {
  // how you define an array
  // let mut numbers: [i32; 4] = [1, 2, 3, 4];
  // how you define a vector
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4]; // don't forget the exclamation mark

  // Re-assign value
  numbers[2] = 20;

  // Add on to vector
  numbers.push(5);
  numbers.push(6);

  // Pop off last value
  numbers.pop();

  println!("{:?}", numbers);

  // Get single val
  println!("Single Value: {}", numbers[0]);

  // Get vector length
  println!("Vector Length: {}", numbers.len());

  // Vectore are stack allocated
  println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

  // Get Slice
  let slice: &[i32] = &numbers[1..3]; // slice comes as an array
  println!("Slice: {:?}", slice);

  // Loop through vector values
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // Loop & mutate values
  for x in numbers.iter_mut() {
    *x *= 2; //mutate each value
    *x += 2; //result is a mapping for every x value so that, x --> 2*x + 2
  }

  println!("Numbers Vec: {:?}", numbers);
}
