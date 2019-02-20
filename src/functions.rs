// Functions - Used to store blocks of code for re-use

pub fn run() {
  greeting("Hello", "Jane");

  // Bind function values to variables
  let get_sum = add(5, 5);
  println!("Sum: {}", get_sum);

  // Closure
  let n3: i32 = 10;
  let add_nums = |n1: i32, n2: i32| n1 + n2 + n3; // it encloses the value of n3 when you define it!
  println!("C Sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) { // always declare the type of the variables
  println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
  n1 + n2 // expression-based language -- this is an implicit return
}
