fn main() {
    let number = 3;

    if number < 5 {
        println!("\n condition was true");
    } else {
        println!("\n condition was false");
    }

    if number % 4 == 0{
        println!("\n Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("\n Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("\n Number is divisible by 2");
    } else {
        println!("\n Number isn't divisible by 4, 3, or 2");
    }

    let other_number = if number == 0 {0} else {99};
    println!("\n The value of other_number is: {}", other_number);

    let mut num = 4;

    println!("\n  Using while loop");
    while num != 0 {
        println!("\n while {num}");
        num -= 1;
    }

    println!("\n  Using for loop");
    let array = [10, 20, 30, 40, 50];
    for element in array {
        println!("\n for {element}");
    }

    println!("\n  Using range");

    for number in 1..=5{
        println!("\n with = {number}");
    }

    println!("\n  Using reverse range");
    
    for number in (1..5).rev() {
        println!("\n reverse {number}");
    }

    println!("\n==Using loop==");

    let mut i = 0;

    loop {
        i += 1;
        if i % 2 == 0{
            continue;
        }
        println!("i = {i}");
        if i >= 10{
            break;
        }
    }

    let result = loop {
        i += 100;
        
        if i >= 100{
            break i * 2;
        }
    };
    println!("\n result - {result}");


    println!("\n ==Labels in loops==");
    let mut count = 0;

    'my_extern: loop{
        println!("Count = {count}");
        let mut remaining = 100;

        loop{
            println!("Remaining = {remaining}");
            if remaining == 97 {
                break;
            }

            println!("Count = {count}");
            if count == 2 {
                break 'my_extern;
            }

            remaining -= 1;
        }
        println!("Add to count");
        count += 1;
    }
    println!("Final count = {count}");

}
