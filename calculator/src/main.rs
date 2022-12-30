use std::env::{args, Args}; // args is a function that returns a struct of type Args. Used for access to command line arguments

fn main() {
    let args = args();

    // println!("{}", args); // This line throws an error since the `println` macro does not know how to format an Args struct
    println!("{:#?}", args); // This is the compiler's advice to extend the formatter. This works.
    println!("Hello, world!");
}
