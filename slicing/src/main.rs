fn main() {
    let s = String::from("mr boombastic in the house");

    let word : &str = first_word(&s);

    println!("The first word is: {}", word);
}

// below function takes &str as argument
// &str is a slice of the String
// using this is more efficient than passing the whole String
// because we don't need to allocate new memory and copy the whole String
// we only need to know where the String starts and ends
// can also pass string vars as arguments using [..]
// also string literals can be passed as arguments using just their name

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
