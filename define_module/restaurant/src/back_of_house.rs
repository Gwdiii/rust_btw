// `Appetizer` is available outside the module
pub enum Appetizer {
    Soup,
    Salad,
}

// `Breakfast` is available outside the module
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast { // Since `Breakfast` has a private field, a public
                 // associated function is required to construct an
                 // instance of Breakfast.
    // `summer` is available outside the module
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}
