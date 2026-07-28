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
    println!("\n Email user1 {}", user1.email);

    let user2 = build_user(String::from("user2@email.com"), String::from("Segundo"));
    let user2 = build_user_with_slice("user2@email.com", "Segundo");

    println!("\n User2: {}", user2.email);

    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("four@email.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("\n user3: {}", user3.email);

    let user4 = User {
        email: String::from("user4@email.com"),
        ..user3
    };
    println!("\n user4: {}", user4.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn build_user_with_slice(email: &str, username: &str) -> User {
    User {
        active: true,
        username: username.to_string(),
        email: email.to_string(),
        sign_in_count: 1,
    }
}
