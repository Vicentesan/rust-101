// Tuples group together values of different types
// Max 12 elements

pub fn run() {
    let tuple: (&str, &str, i8) = ("one", "two", 3);

    println!("{:?}", tuple);
}
