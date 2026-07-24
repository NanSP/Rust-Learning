fn factorial_classic(n: i64) -> i64 {
    let mut factorial = 1;

    for i in 2..=n {
        factorial *= i;
    }
    factorial
}

fn factorial_recursive(n: i64) -> i64 {
    if n <= 1 {
        return 1;
    }
    n * factorial_recursive(n - 1)
}

fn factorial_iterator(n: i64) -> i64 {
    (1..=n).product()
}

fn main() {
    let x: i64 = 4;

    println!("Factorial classic of {} is {}", x, factorial_classic(x));
    println!("Factorial recursive of {} is {}", x, factorial_recursive(x));
    println!("Factorial iterator of {} is {}", x, factorial_iterator(x));
}
