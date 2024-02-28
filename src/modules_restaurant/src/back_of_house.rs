fn _fix_incorrect_order() {
    _cook_order();
    super::_deliver_order(); // use "super::"" since function is defined in parent module
}

fn _cook_order() {}

pub struct Breakfast {
    pub toast: String, // need to set "pub" here
    _seasonal_fruit: String, // not public since chef chooses which one you get
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            _seasonal_fruit: String::from("peaches"),
        }
    }
}
#[derive(Debug)]
pub enum Appetizer {
    Soup, // default is public!!!
    Salad,
}
