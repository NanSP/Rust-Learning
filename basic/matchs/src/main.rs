enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

enum CoinYear {
    Penny,
    Nickel,
    Dime,
    Quarter(usize),
}

fn value_in_cents2(coin: CoinYear) -> usize {
    match coin {
        CoinYear::Penny => 1,
        CoinYear::Nickel => 5,
        CoinYear::Dime => 10,
        CoinYear::Quarter(year) => {
            println!("Quarter from year {}", year);
            25
        }
    }
}

fn main() {
    let m1 = CoinYear::Penny;
    let m2 = CoinYear::Quarter(1999);

    println!("Value of m1: {}", value_in_cents2(m1));
    println!("Value of m2: {}", value_in_cents2(m2));
}
