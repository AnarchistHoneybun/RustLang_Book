struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("example@something.com"),
        username: String::from("example"),
        active: true,
        sign_in_count: 1,
    };

    println!("User 1: {}", user1.email);

    // TODO make another user using field init shorthand
    let user2 = make_user(String::from("newexample@something.com"), String::from("newexample"));

    println!("User 2: {}", user2.email);
    
}

fn make_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
