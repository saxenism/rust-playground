mod args;
use args::Args;

fn main() {
    let args: Args = Args {
        image_1: String::new(), // This `new` function has been implemented specifically for this `String` struct.
        image_2: String::new(),
        output: String::new()
    };
    
    println!("Hello, world!");
}
