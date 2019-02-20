// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
  let mut numbers: [i32; 4] = [1, 2, 3, 4];

  // Re-assign value
  numbers[2] = 20; // you need to declare it as mutable though

  println!("{:?}", numbers);

  // Get single val
  println!("Single Value: {}", numbers[0]);

  // Get array length
  println!("Array Length: {}", numbers.len());

  // Arrays are stack allocated and not heap allocated -- fixed size
  println!("Array occupies {} bytes", mem::size_of_val(&numbers)); // you use the mem standard library and the reference of the array
  // Due to 4 i32 characters in the array it occupies 16 bytes of memory

  // Get Slice
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice);
}
