// Tuples group together values of different types
// Can contain max 12 elements

pub fn run() {
  let person: (&str, &str, i8) = ("Brad", "Mass", 37);

  println!("{} is from {} and is {}", person.0, person.1, person.2); // person.0 gets the zeroth element
}
