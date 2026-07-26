fn main() {
    let s = String::from("Hello world");

    let s1 = &s[0..5];
    let s2 = &s[6..11];
    let s3 = &s[..2];
    let s4 = &s[3..];

    let slit = "Hi, everyone!";

    println!(
        "s1: {}, s2: {}, s3: {}, s4: {}, slit: {}",
        s1, s2, s3, s4, slit
    );

    let d = String::from("Hello Jupyter, Mars, Pluton!");

    let word = first_word(&d);

    println!("The first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
