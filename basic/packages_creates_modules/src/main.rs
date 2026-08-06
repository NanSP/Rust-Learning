mod front_of_house;

fn other_function() -> String {
    String::from("Other function")
}

fn main() {
    println!("Function in main: {}", other_function());

    println!(
        "Function of the module 'front_of_house': {}",
        front_of_house::init_front_of_house()
    );

    println!(
        "Function of the submodule 'hosting': {}",
        front_of_house::hosting::add_to_waitlist()
    );
}
