fn main() {
    let s = String::from("hello");

    receive_ownership(s);
    //receive_ownership(s.clone()); <-- a way to 'copy' a value of a ownership
    //println!("{}", s); //error because the value of s was moved for the function receive_ownership

    println!("Hello, world!");
}

fn receive_ownership(a_string: String) {
    println!("{}", a_string);
}
