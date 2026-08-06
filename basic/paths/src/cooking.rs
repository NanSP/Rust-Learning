use crate::hosting;

use crate::hosting::Breakfast;

use std::fmt;
use std::io;

fn preapre_food() -> String {
    String::from("Food prepared")
}

pub fn clean_up() -> String {
    crate::other_function();
    super::other_function();

    println!("Absolute path: {}", crate::hosting::add_to_waitlist());

    println!("Absolute path: {}", super::hosting::add_to_waitlist());

    let bb = Breakfast::new();

    println!("Using use: {}", hosting::add_to_waitlist());

    String::from("Cooking clean up")
}
