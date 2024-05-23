mod front_of_house;

mod back_of_house;

fn deliver_order() {}

// This allows someone to call
//   restaurant::hosting::add_to_waitlist()
// instead of
//   restaurant::front_of_house::hosting::add_to_waitlist()
pub use crate::front_of_house::hosting;

// Grouping uses that pull from same module
use crate::back_of_house::{Appetizer, Breakfast};

pub fn eat_at_restaurant() {
    // Absolute path
    //crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    //front_of_house::hosting::add_to_waitlist();

    // Using shortcut
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast, please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;
}
