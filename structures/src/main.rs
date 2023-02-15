struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("example@something.com"),
        username: String::from("example"),
        active: true,
        sign_in_count: 1,
    };

    println!("User 1: {}", user1.email);
}
