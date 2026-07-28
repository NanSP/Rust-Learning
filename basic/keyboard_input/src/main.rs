use std::io;

fn size_words_v1() {
    println!("\n ==SIZE OF WORDS v1==");

    let mut words_list = Vec::new();

    loop {
        print!("\n ------[V1] Write a word or just press enter for cancel------");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Error read keyboard.");
        line = line.trim().to_string();

        if line.len() == 0 {
            break;
        } else {
            println!("Read:{}", line);
            words_list.push(line);
        }
    }

    for p in words_list {
        println!("{}", p);
    }
}

fn size_words_v2() {
    println!("\n ==SIZE OF WORDS v2");

    let mut words_list = Vec::new();

    loop {
        print!("\n ------[V2] Write a word or just press enter for cancel------");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Error read keyboard.");
        line = line.trim().to_string();

        if line.len() == 0 {
            break;
        } else {
            let words = line.split_whitespace();

            for p in words {
                words_list.push(p.trim().to_string());
            }
        }
    }
    print!("\n Were typed {} words", words_list.len());

    let mut min = 99999;
    let mut max = 0;
    let mut total = 0;

    for p in words_list.iter() {
        let size = p.chars().count();
        total += size;

        if size < min {
            min = size;
        }

        if size > max {
            max = size;
        }
        println!("{}", p);
    }
    if words_list.len() > 0 {
        println!(
            "Min = {} Medium= {} Max= {}",
            min,
            total / words_list.len(),
            max
        );
    }
}

fn main() {
    size_words_v1();
    size_words_v2();
}
