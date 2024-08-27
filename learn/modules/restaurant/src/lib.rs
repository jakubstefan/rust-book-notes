#![allow(dead_code)] // do not warn that is never used
#![allow(unused_variables)]

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super builds relative paths that begin in the parent module
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String, // public field
        seasonal_fruit: String, // private field
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // we only need the 'pub' before the 'enum' keyword to make an enum public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// if we use the following, we can later directly use hosting::add_to_waitlist();
// use create::front_of_house::hosting;

/* While for functions, the idiomatic way is to bring its parent module into scope (as above),
 * for structs, enums and other items it's idiomatic to specify the full path
 */
use std::collections::HashMap;

/* If we wanted to re-export the hosting module, so add_to_waitlist function could
 * be called from external code as restaurant::hosting::add_to_waitlist()
 */
//pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    let mut map = HashMap::new();
    map.insert(1, 2);
}

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}

// You can combine 'use' statements
// use std::{cmp::Ordering, io};
/* which is the same as
use std::cmp::Ordering;
use std::io;
*/

// use std::io::{self, Write};
/* which is the same as
use std::io;
use std::io::Write;
*/

// The Glob Operator brings all public items
// use std::collections::*;
