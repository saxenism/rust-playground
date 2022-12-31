use std::env::{args, Args}; // args is a function that returns a struct of type Args. Used for access to command line arguments

fn main() {
    //let args = args(); //Although this is fine and won't throw any errors, I will follow the convention in the next line denoting some kinda type safety
    // let args: Args = args(); // This says the exact same thing as the above line along with the fact that variable args is of type Args
    let mut args: Args = args(); // Commented out line 5, because `args` by default was immutable and we could have used the function `nth` on a immutable Args.

    // println!("{}", args); // This line throws an error since the `println` macro does not know how to format an Args struct
    println!("1. {:#?}", args); // This is the compiler's advice to extend the formatter. This works.
    println!("2. Hello, world!");

    /*
    // Let's use the `nth` method that exists on the Args struct
    let first_wrapped = args.nth(0); // Accessing the first argument that is passed on the command line
    println!("3. {:#?}", first_wrapped); // This throws an error: cannot borrow `args` as mutable, as it is not declared as mutable
                              // So we make changes in line 5 and make `args` as mutable.
                            // In this print statment, the value is wrapped in `Some()`.
    */
    
    let first = args.nth(1).unwrap(); // Since we wanted only the value, we unwrapped first.
    println!("4. {:#?}", first); 

    // Not passing 2 and 3 below in the `nth` function since calling the `nth` function once moves the interal iterator by 1 and that is
    // why we had to declare `args` as `mut` in the first place.

    let operator = args.nth(0).unwrap(); 
    let second = args.nth(0).unwrap();

    println!("5. {} {} {}", first, operator, second);

    // Now, unlike some other languages we cannot do simple arithmetic operations on strings and we would need to convert these strings into numbers
    /*
    let first_number = first.parse().unwrap();
    let second_number = second.parse().unwrap();
    // The above line 33 and 34 will throw an error since the `parse` method does not know what is the resulting type you expect when it parses the input string.
    // Therefore, the lines below 39, 40 show the correct method.
    */

    let first_number: f32 = first.parse().unwrap();
    let second_number: f32 = second.parse().unwrap();

    // let result: f32 = operate(operator, first_number, second_number); // This will throw an error since function `operate` expects `operator` to be a char value but `operator` is a `String` value right now.
                                                                    //  we need to convert the `operator` string into char. Which we do in line 45.

    let result: f32 = operate(operator.chars().next().unwrap(), first_number, second_number); // chars is a method on Strings and next is basically a method to access the iterator and unwrap simply presents the value of the char

    println!("6. {} {}", first_number, second_number);
    println!("The result of this operation is\n {:#?}", result);

    println!("{:#?}", output(first_number, operator.chars().next().unwrap(), second_number, result));
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    if operator == '+'  {
        first_number + second_number // No `return` keyword and `;` required in Rust, as Rust has implicit return policy
    } else if operator == '-'  { 
        first_number - second_number
    } else if  operator == '*'  { // Also we don't need to have any parentheses around the if and else if keyword conditions
        first_number * second_number
    } else if  operator == '/'  {
        first_number / second_number
    } else {
        0.0 // Ideally we would have wanted to panick or return an error here, but since the return type of this function is f32, we would make do with 0.0 for now.
    }
}

// Formatting the output
fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!("{} {} {} = {}", first_number, operator, second_number, result)
}