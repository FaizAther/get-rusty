#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod front_house {
    pub mod hosting {
        pub fn add_waitlist() {}
        fn seat_table() {}
    }

    pub mod serving {
        fn take_order() {}
        pub fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_house {
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
}

use crate::front_house::hosting;

use self::back_house::Breakfast;

use rand::{Rng, CryptoRng, Error};  // Nested paths

use std::io::{self, Write};

use std::io::*;

pub fn eat() {
    //  Absolute
    crate::front_house::hosting::add_waitlist();
    
    //  Relative
    front_house::hosting::add_waitlist();

    let mut meal: back_house::Breakfast = back_house::Breakfast::summer("Dark");

    meal.toast = String::from("Light");

    let order1 = back_house::Appetizer::Salad; // Both enums are public
    let order2 = back_house::Appetizer::Soup;

    // use keword no need to specify path now
    hosting::add_waitlist();

    // use keyword no need to speciy path now
    let order3 = Breakfast::summer("None");

    let secret_num: i32 = rand::thread_rng().gen_range(0..100);
}

