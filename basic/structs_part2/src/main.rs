#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}

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

    let scale = 2;

    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        heigth: 50,
    };

    println!("The rectangle area is {} pixels", area(&rect1));

    println!("\n\n Area2");
    area2(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.heigth
}

fn area2(rectangle: &Rectangle) -> u32 {
    dbg!(&rectangle);
    dbg!(rectangle.width * rectangle.heigth)
}
