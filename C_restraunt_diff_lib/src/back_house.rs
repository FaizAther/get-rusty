pub enum Appetizer {
    Soup,
    Salad
}

pub struct Breakfast {
    pub toast: String,
    fruit: String
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            fruit: String::from("Orange")
        }
    }
}

fn fix_order() {
    cook_order();
    super::front_house::serving::serve_order()
}

fn cook_order() {}