// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let hello_immutable_primitive_type = "Hello";

    println!("{}", hello_immutable_primitive_type);

    let mut hello = String::from("Hello ");

    // Get length -- this works for either type
    println!("Length: {}", hello.len());

    // Push char -- need for mutable variable
    hello.push('W');

    // Push string
    hello.push_str("orld!");

    // Capacity in bytes
    println!("Capacity, number of bytes: {}", hello.capacity());

    // Check if empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World' {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len()); // if it passes, nothing happens, it just passes, otherwise, the compiler panics
    assert_eq!(10, s.capacity());

    println!("{}", s);
}
