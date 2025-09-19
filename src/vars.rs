// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Vicente";
    let mut mutable_age = 15;

    mutable_age = mutable_age + 1;

    println!("My name is {}, and I am {} years old!", name, mutable_age);

    // Define constant

    const ID: i32 = 001;

    println!("ID: {}", ID);

    // Assign multiple variables at once
    let (my_name, my_age): (&str, u8) = ("Vicente", 16);

    println!("My name is {}, and I am {} years old!", my_name, my_age);
}
