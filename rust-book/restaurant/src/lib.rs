// Remember: Packages have crates (binary or library) and crates have modules that is used to organise chunks of code and manage permissions
// Modules can contain other modules inside of them. They could also contain structs, enums, traits, constants, etc.
// Structuring your *Restaurant* package in such modules makes sense or keeps everything organised

/* 
mod front_of_house {
    
    // By default every child module and everything inside of it is private from the perspective of the parent module.
    // On the flip side, child modules can see anything that's defined in their parent modules.
    // This allows us to hide implementation details and only expose to the world, what we need to.
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
*/

// This is what the module tree for this structure looks like:

// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

/* 
// Simplied `front_of_house` module showcasing usage of paths and visibility
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    } 
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // And btw, the keyword `super` can be used to reference the parent module as well.
}
*/

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("mango")
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat"); // To do this, we had to make the `toast` as a public field
    // That also means that we cannot create a Breakfast struct directly inside of `eat_at_restaurant` because the `season_fruit` is a private property.
    // It has to be done via the function `Breakfast::summer`.

    //Eg. This does not work, We get the error: field is private
    /*
    let meal2 = back_of_house::Breakfast {
        toast: String::from("some random toast"),
        seasonal_fruit: String::from("Strawberries"),
    }
     */

    // This wouldn't work until `Appetizer` is public.
    // Interesting thing to note here is that, the enum fields are by default public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// And ofc, as we had already seen that we can the use the `use` keyword to bring some particular
// path into scope

// The level of path that should be brought into scope is based on balancing brevity and clarity with
// avoiding confusions about the imported function or whatever.
/*
For example:
use std::fmt;
use std::io;

fn function1() -> fmt::Result {

}

fn function2() -> io::Result {

}

Here it would have created a lot of ambiguity had we imported std::fmt::Result and std::io::Result
OR
We could have re-named whatever we were importing such as:
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;
*/

// If we want external code to be able to access Soup, we need to use the pub keyword (for re-exporting)
// So external code can reference Soup as well.
pub use crate::back_of_house::Appetizer::Soup; // Absolute Path
use back_of_house::Appetizer::Salad; // Relative Path


use rand::Rng;
use rand::{CryptoRng, ErrorKind::Transient}; //Both these imports could have been different lines, but we simplified those imports into a single line by grouping the common path by using something called NESTED PATHS.

pub fn eat() {
    // Example of referencing of an external library code.
    let randomNumber = rand::thread_rng().gen_range(1, 101);

    let order1 = Soup;
    let order2 = Salad;
}

mod front_of_house; // This tells Rust, define the `front_of_house` module here, but get the contents from a different file with the same name

pub use crate::front_of_house::hosting;

pub fn eat_eat() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
