pub fn run() {
    // Basic Print
    println!("Hello from the print.rs file!");

    // Print with arguments
    println!("Hello {} from print.rs {}", "Vicente", "file");

    // Print with numbered arguments
    println!(
        "Hello {0} from print.rs {1}, and {0} likes to {2}",
        "Vicente", "file", "code"
    );

    // Print with named arguments
    println!(
        "Hello {name}, from print.rs {file}",
        name = "Vicente",
        file = "file"
    );

    // Print with Placeholder
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug traits
    println!("{:?}", (12, true, "Hello"));

    // Baisc math
    println!("10 + 10 = {}", 10 + 10)
}
