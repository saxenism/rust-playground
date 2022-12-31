mod args;
use args::Args;

fn main() {
    /*
        // This method of declaring a new Args struct will turn out to be quite uncomfortable. So we should abstract it a function implemented for this struct specifically.
        let args: Args = Args {
            image_1: String::new(), // This `new` function has been implemented specifically for this `String` struct.
            image_2: String::new(),
            output: String::new()
        };
    */
    let args: Args = Args::new();

    println!("Hello, world!");
}
