use std::io;

fn main() {
    println!("Enter a number: ");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = n.trim().parse().expect("Please type a number!");

    println!("The {}th Fibonacci number is {}", n, fibo(n));
}

fn fibo(n: u32) -> u32 {
    let mut a = 1;
    let mut b = 1;
    let mut result = 0;

    for _ in 0..n {
        result = a + b;
        a = b;
        b = result;
    }

    result
}
