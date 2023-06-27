mod front_of_house {
    // `hosting` is available outside the module
    pub mod hosting {
        // `add_to_waitlist` is available outside the module
        pub fn add_to_waitlist() {}
    }
}

mod back_of_house {
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
}

mod customer {

    // hosting, Appetizer, and Breakfast are available in this scope
    use crate::front_of_house::hosting;
    use crate::back_of_house::{Appetizer, Breakfast};

    // `eat_at_restaurant` is available outside the module
    pub fn eat_at_restaurant() {
        // without the use statements on ln 38,
        // `hosting::add_to_waitlist` would not work here
        hosting::add_to_waitlist();

        // without the use statement on ln 39,
        // `Appetizer::*` would not work here
        let _order1 = Appetizer::Soup;
        let _order2 = Appetizer::Salad;

        // Order a breakfast in the summer with Rye toast
        let mut meal = Breakfast::summer("Rye");

        // without the use statement on ln 39,
        // `Breakfast::summer` would not work here
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // The next line won't compile if uncommented; we're not allowed
        // to see or modify the `seasonal_fruit` that comes with the meal
        // meal.seasonal_fruit = String::from("blueberries");
    }
}
