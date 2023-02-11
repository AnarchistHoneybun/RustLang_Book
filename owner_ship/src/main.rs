fn main() {
    let s1 = String::from("Bing Chilling");
    // ! clone operations, however, are costly
    let s2 = s1.clone();

    println!("s1: {}, s2: {}", s1, s2);
    ////println!("s1: {}", s1);
}
