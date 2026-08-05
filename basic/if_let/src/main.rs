enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(usize),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let config_max = Some(3);

    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let coin = Coin::Quarter(1999);
    let mut count = 0;

    match coin {
        Coin::Quarter(year) => println!("Year of the quarter is: {:?}!", year),
        _ => count += 1,
    }

    if let Coin::Quarter(year) = coin {
        println!("Year of the quarter is: {:?}!", year);
    } else {
        count += 1;
    }
}
