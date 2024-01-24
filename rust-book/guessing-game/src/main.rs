use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess");

        // By default all the variables in Rust are immutable and if we want them
        // to be immutable, we append their declaration with `mut`. **good safety measure**
        let mut guess:String = String::new();
    
        io::stdin().read_line(&mut guess).expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Number expected as input".yellow());
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("{}", "Too small".red());
            },
            Ordering::Greater => {
                println!("{}", "Too big".red());
            },
            Ordering::Equal => {
                println!("{}", "You win".green());
                break;
            }
        }
    }
}
