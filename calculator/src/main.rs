use std::env::{args, Args};

fn main() {
    /*
    let args: Args = args();
    // println!("{}", args); // Args cannot be formatted with the default formatter.
                          // Basically, the println macro does not know how to format the Args struct

    println!("{:#?}", args); // Compiler's suggestion
    */

    /*
    let mut args: Args = args();

    let first_argument = args.nth(1).unwrap(); // Returns the exact phrase of argument when it exists, otherwise panics
    let first_argument_wrapped = args.nth(1); // Returns None || Returns Some(argumentValue)

    println!("{:?}", args);
    println!("{:?}", first_argument);
    println!("{:?}", first_argument_wrapped);
    */

    // This printing of operator and second_argument will panic because the nth function shifts the operator every time it is called
    /*
    let mut args: Args = args();

    let first_argument = args.nth(1).unwrap();
    let operator = args.nth(2).unwrap();
    let second_argument = args.nth(3).unwrap();

    println!("{:#?} {:#?} {:#?}", first_argument, operator, second_argument);
    */

    let mut args: Args = args();

    let first_argument = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second_argument = args.nth(0).unwrap();

    println!("{} {} {}", first_argument, operator, second_argument);

    let first_number: f32 = first_argument.parse().unwrap();
    let second_number: f32 = second_argument.parse().unwrap();

    println!("{} {} ", first_number, second_number);

    let result = operate(operator, first_number, second_number);
    let result_ifelse: f32 = operate_ifelse(operator, first_number, second_number);

    println!("Result: {}", result);
    println!("Result ifElse: {}", result_ifelse);
}

fn operate_ifelse(operator: char, first_number: f32, second_number: f32) -> f32 {
    if operator == '+' {
        first_number + second_number
    } else if operator == '-' {
        first_number - second_number
    } else if operator == '*' {
        first_number * second_number
    } else if operator == '/' {
        first_number / second_number
    } else {
        0.0
    }
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' =>  first_number + second_number,
        '-' =>  first_number - second_number,
        '*' | 'x' | 'X'  =>  first_number * second_number,
        '/' =>  first_number / second_number,
        // _ => 0.0 // Returning 0 in case of invalid operator does not make much sense, so we will panic
        _ => panic!("Invalid operator used")
    }
}