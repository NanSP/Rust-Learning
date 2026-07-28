struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("username1"),
        email: String::from("user@email.com"),
        sign_in_count: 1,
    };
    println!("\n Email user1: {}", user1.email);

    user1.email = String::from("usermail@email.com");
    user1.sign_in_count += 1;
    println!("\n Email user1 {}", user1.email);

    let user2 = build_user(String::from("user2@email.com"), String::from("Segundo"));
    println!("\n Email user2: {}", user2.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
