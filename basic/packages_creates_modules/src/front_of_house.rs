pub mod hosting;

mod serving;

pub fn init_front_of_house() -> String {
    String::from("Front of house initialized")
}

fn ways_callings() {
    crate::other_function();
    super::other_function();
}
