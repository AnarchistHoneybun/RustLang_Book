fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    // ! mutable and immutable references in the same scope
    // ! Rust disallows this

    // ! multiple mutable references to the same location
    // ! Rust disallows this
    println!("{}", r3);
}
