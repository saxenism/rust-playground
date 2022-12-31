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
    /*
        // println!("{}", args); // This throws an error, since println macro does not know how to format the Args struct type.
        // println!("{:#?}", args); // Since the above did not work, let's try extending the formatter.
        // This also did not work since the trait `Debug` was not implemented for `args::Args`. So we will try printing again after implementing the trait Debug for Args
    */
    println!("{:#?}", args);
    println!("Hello, world!");
}
