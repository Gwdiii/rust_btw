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
