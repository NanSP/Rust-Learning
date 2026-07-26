fn main() {
    let s = String::from("Hi");
    //change1(&s);

    let mut x = String::from("Hello");
    change2(&mut x);
}

/* === WRONG===
fn change1(some_string: &String) {
    some_string.push_str(", everyone!");
}
*/
fn change2(some_string: &mut String) {
    some_string.push_str(", world");
    print!("{}", some_string);
}
