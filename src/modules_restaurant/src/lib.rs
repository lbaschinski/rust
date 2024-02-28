mod front_of_house;

fn _deliver_order() {}

mod back_of_house;

// bring the parent module into scope, so when doing "hosting::add_to_waitlist()"
// it is clear where the function comes from (not locally defined therefore)
// add pub if users of our module should be able to use it too (re-exporting)
// this will shorten "restaurant::front_of_house::hosting::add_to_waitlist()"
// to "restaurant::hosting::add_to_waitlist()"
pub use crate::front_of_house::hosting;
// BUT, use the full path for structs/emums
use std::collections::HashMap;
// If there are two items with the same name you need to "use" parent module
use std::fmt;
// or use the "as" keyword
use std::io::Result as IoResult;
// nested paths
#[allow(unused_imports)]
use std::{cmp::Ordering, hash};
#[allow(unused_imports)]
use std::io::{self, Write};
// glob operator
#[allow(unused_imports)]
use std::collections::*;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
    // Path possible thanks to the usage of "use" above
    hosting::add_to_waitlist();

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
    println!("{:?}, {:?}", order1, order2);

    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("{:?}", map);

    // two different Result types, either use parent module or alias to resolve name clash
    fn _function1() -> fmt::Result {
        Ok(())
    }
    fn _function2() -> IoResult<()> {
        Ok(())
    }
}
