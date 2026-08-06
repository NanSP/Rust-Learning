fn take_order() -> String {
    String::from("Order taken")
}

fn serve_order() -> String {
    String::from("Order served")
}

fn take_payment() -> String {
    println!(
        "Funtion of the module 'front_of_house': {}",
        super::init_front_of_house()
    );
    println!(
        "Function of the submodule 'hosting': {}",
        super::hosting::add_to_waitlist()
    );

    String::from("Payment taken")
}
