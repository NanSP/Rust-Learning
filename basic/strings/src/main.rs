fn main() {
    let s = String::new();
    println!("P1: s >>>{s}");

    let s = String::from("Initial");
    println!("P1: s >>>{s}");

    let data = "initial content";
    let s = data.to_string();
    println!("P2: s >>>{s}");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("P3: s >>>{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("P4: s3>>> {s3}");
}
