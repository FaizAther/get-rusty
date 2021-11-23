#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod front_house;

mod back_house;

use crate::front_house::hosting;

use self::back_house::Breakfast;

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
}

