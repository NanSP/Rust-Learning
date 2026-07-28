#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct AlwaysEqual;

struct User {
    active: bool,
    username: String,
    email1: String,
    sign_in_count: u64,
}

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(1, 2, 3);

    println!("Origin: {:?}", origin);

    let new: Color;
    new = black;

    let unit_struct = AlwaysEqual;
    println!("Unit_struct: {:?}", unit_struct);
}
