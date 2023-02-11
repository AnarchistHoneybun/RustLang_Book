fn main() {
    let s1 = String::from("buongiorno");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len() // len() returns the length of a String

}
