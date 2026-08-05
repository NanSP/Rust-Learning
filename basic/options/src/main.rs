fn sum_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn sum_options(x: Option<i32>, y: Option<i32>) -> Option<i32> {
    match (x, y) {
        (Some(i), Some(j)) => Some(i + j),
        (Some(i), None) => None,
        (None, Some(j)) => None,
        (None, None) => None,
    }
}

fn main() {
    let num5 = Some(5);
    let not_num: Option<i32> = None;

    let some_char = Some('e');

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    //let sum1 = x + y;

    println!("num5: {:?}", num5);
    println!("Sum one with number 5: {:?}", sum_one(num5));
    println!("------");

    println!("not_num: {:?}", not_num);
    println!("Sum one with None: {:?}", sum_one(not_num));
    println!("------");

    println!("Sum option: {:?}", sum_options(num5, num5));
}
