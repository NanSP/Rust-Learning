fn main() {
    let word = String::from("apple");

    let len1 = length_of_move(word.clone()); //keep the owner on variable 'word'
    println!("The length of '{}' is {}.", word, len1);

    let len2 = length_of_reference(&word); //use the reference of a String with '&' to borrowing the value
    println!("The length of '{}' is {}.", word, len2);
}

fn length_of_move(s: String) -> usize {
    s.len()
}

fn length_of_reference(s: &String) -> usize {
    s.len()
}
