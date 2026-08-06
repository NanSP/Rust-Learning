use rand::Rng;

use std::collections::HashMap;

use std::{self, cmp::Ordering, io};

mod cooking;
mod hosting;
mod serving;

fn other_function() -> String {
    String::from("Other function")
}

fn main() {
    println!("Function from this module: {}", other_function());

    println!("Cooking clean up: {}", cooking::clean_up());
    println!("Serving clean up: {}", serving::clean_up());

    println!("Random number: {}", rand::thread_rng().gen_range(1..=100));
}
