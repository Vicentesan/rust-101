// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let primitive_hello = "Hello";

    println!("Primitive Hello: {}", primitive_hello);

    // Get length
    let string_hello = String::from("Hello");

    println!("Length: {}", string_hello.len());

    // Mutable string
    let mut mutable_string = String::from("Hello");

    // Push char
    mutable_string.push(',');

    // Push string
    mutable_string.push_str(" World!");

    println!("Mutable String: {}", mutable_string);

    // Capacity in bytes
    println!("Capacity: {}", mutable_string.capacity());

    // Check if empty
    println!("Is Empty: {}", mutable_string.is_empty());

    // Contains
    println!("Contains 'World': {}", mutable_string.contains("World"));

    // Replace
    println!("Replace: {}", mutable_string.replace("World", "There"));

    // Loop through string by whitespace
    for word in mutable_string.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut capacity_string = String::with_capacity(10);
    capacity_string.push('a');
    capacity_string.push('b');

    println!("Capacity String: {}", capacity_string);
    println!("Capacity String: {}", capacity_string.capacity());

    // Assert testing
    assert_eq!(2, capacity_string.len());
    assert_eq!(10, capacity_string.capacity());
}
