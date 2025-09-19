/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run() {
    // Default integer type is i32
    let x = 1;

    // Default float type is f64
    let y = 2.5;

    {
        let y = "3";
        // This creates a new variable y that shadows the outer y within this scope
        println!("Inner y: {}", y); // Prints 3
    }

    println!("{}", y); // Prints 2.5 because we're back to the original y

    // Add explicit type
    let z: i64 = 2147483648;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    println!("{:?}", (is_active, x, y, z));

    // Get bool from expression
    let is_greater = 10 > 5;

    println!("{}", is_greater);

    // Char
    let a = 'a';

    println!("{}", a);

    let smiling_face = '\u{1F600}';

    println!("{}", smiling_face);
}
