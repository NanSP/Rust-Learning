struct KnownPrimes {
    primes: Vec<u64>,
    gap_begin: u64,
    gap_end: u64,
}

fn test_prime_v1(target: u64) -> bool {
    if target <= 1 {
        return false;
    }
    let mut divider = 2;
    while divider * divider <= target {
        if target % divider == 0 {
            return false;
        }
        divider += 1;
    }
    true
}

fn test_prime_v2(target: u64) -> bool {
    if target <= 1 {
        return false;
    } else if target == 2 || target == 3 || target == 5 || target == 7 {
        return true;
    } else if target % 2 == 0 {
        return false;
    } else if target % 3 == 0 {
        return false;
    } else if target % 5 == 0 {
        return false;
    } else if target % 7 == 0 {
        return false;
    }

    let mut divider = 11;
    while divider * divider <= target {
        if target % divider == 0 {
            return false;
        }
        divider += 2;
    }
    true
}

fn test_prime_v3(known_primes: &KnownPrimes, target: u64) -> bool {
    if known_primes.gap_end * known_primes.gap_end < target || known_primes.gap_begin > 0 {
        panic!("test_prime_v3 call whitout prime numbers");
    }

    if target == 1 {
        return false;
    }

    for divider in known_primes.primes.iter() {
        if divider * divider > target {
            break;
        }
        if target % divider == 0 {
            return false;
        }
    }
    true
}

fn main() {
    println!("\n prime_number_v1({}) -> {}", 1, test_prime_v1(1));
    println!("\n prime_number_v1({}) -> {}", 2, test_prime_v1(2));
    println!("\n prime_number_v1({}) -> {}", 3, test_prime_v1(3));
    println!("\n prime_number_v1({}) -> {}", 8, test_prime_v1(8));
    println!("\n prime_number_v1({}) -> {}", 97, test_prime_v1(97));

    println!("\n prime_number_v2({}) -> {}", 1, test_prime_v2(1));
    println!("\n prime_number_v2({}) -> {}", 2, test_prime_v2(2));
    println!("\n prime_number_v2({}) -> {}", 3, test_prime_v2(3));
    println!("\n prime_number_v2({}) -> {}", 8, test_prime_v2(8));
    println!("\n prime_number_v2({}) -> {}", 97, test_prime_v2(97));

    let known_primes = KnownPrimes {
        primes: vec![2, 3, 5, 7],
        gap_begin: 0,
        gap_end: 10,
    };

    println!(
        "\n prime_number_v3({}) -> {}",
        1,
        test_prime_v3(&known_primes, 1)
    );
    println!(
        "\n prime_number_v3({}) -> {}",
        2,
        test_prime_v3(&known_primes, 2)
    );
    println!(
        "\n prime_number_v3({}) -> {}",
        3,
        test_prime_v3(&known_primes, 3)
    );
    println!(
        "\n prime_number_v3({}) -> {}",
        8,
        test_prime_v3(&known_primes, 8)
    );
    println!(
        "\n prime_number_v3({}) -> {}",
        97,
        test_prime_v3(&known_primes, 97)
    );
}
