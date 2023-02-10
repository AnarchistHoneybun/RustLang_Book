use std::io;

fn main() {
    println!("Input Fahrenheit temp: ");

    let mut fahrenheit = String::new();

    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: f32 = fahrenheit.trim().parse().expect("Please type a number!");

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

    println!("{}°F is {}°C", fahrenheit, celsius);
}
