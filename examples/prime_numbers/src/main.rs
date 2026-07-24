fn prime_number_while(num: u32) -> bool {
    if num <= 1 {
        return false;
    }

    let limit = (num as f64).sqrt() as u32;
    let mut d = 2;
    while d <= limit {
        if num % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn prime_number_for(num: u32) -> bool {
    if num <= 1 {
        return false;
    }

    let limit = (num as f64).sqrt() as u32;
    for d in 2..=limit {
        if num % d == 0 {
            return false;
        }
    }
    true
}

fn prime_number_for_v2(num: u32) -> bool {
    if num <= 1 {
        return false;
    } else if num == 2 {
        return true;
    } else if num == 3 {
        return true;
    } else if num % 2 == 0 {
        return false;
    } else if num % 3 == 0 {
        return false;
    }

    let limit = (num as f64).sqrt() as u32;
    for d in (5..=limit).step_by(2) {
        println!("    d: {} ", d);
        if num % d == 0 {
            return false;
        }
    }
    true
}

fn main() {
    println!("\nprime_number_while ({}) -> {}", 1, prime_number_while(1));
    println!("\nprime_number_while ({}) -> {}", 2, prime_number_while(2));
    println!("\nprime_number_while ({}) -> {}", 3, prime_number_while(3));
    println!("\nprime_number_while ({}) -> {}", 8, prime_number_while(8));
    println!(
        "\nprime_number_while ({}) -> {}",
        97,
        prime_number_while(97)
    );

    println!("\nprime_number_for ({}) -> {}", 1, prime_number_for(1));
    println!("\nprime_number_for ({}) -> {}", 2, prime_number_for(2));
    println!("\nprime_number_for ({}) -> {}", 3, prime_number_for(3));
    println!("\nprime_number_for ({}) -> {}", 8, prime_number_for(8));
    println!("\nprime_number_for ({}) -> {}", 97, prime_number_for(97));

    println!(
        "\nprime_number_for_v2 ({}) -> {}",
        1,
        prime_number_for_v2(1)
    );
    println!(
        "\nprime_number_for_v2 ({}) -> {}",
        2,
        prime_number_for_v2(2)
    );
    println!(
        "\nprime_number_for_v2 ({}) -> {}",
        3,
        prime_number_for_v2(3)
    );
    println!(
        "\nprime_number_for_v2 ({}) -> {}",
        8,
        prime_number_for_v2(8)
    );
    println!(
        "\nprime_number_for_v2 ({}) -> {}",
        97,
        prime_number_for_v2(97)
    );
}
