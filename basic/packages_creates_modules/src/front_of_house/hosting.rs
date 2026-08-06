pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

pub enum Appetizer {
    Soup,
    Salad,
}

pub fn add_to_waitlist() -> String {
    String::from("Added to waitlist")
}

fn seat_at_table() -> String {
    String::from("Seated at table")
}
