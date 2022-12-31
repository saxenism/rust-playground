fn get_nth_arg(n: usize) -> String {
    std::env::args().nth(n).unwrap()
}

// For this project, we have declared a custom struct called Args and it's members are 2 strings (image1 and image2) along with an output of type String again.
// The struct is marked public (pub) so that it can be called in main.rs
// The members of this struct have to marked as public so they can be accessed outside of this module
#[derive(Debug)] // Reason for adding this can be seen in line 17 of main.rs
pub struct Args {
    pub image_1: String,
    pub image_2: String,
    pub output: String
}

impl Args {
    // Again since we want the new function to be accessible to people outside of this module(file) too, we declare it as public
    pub fn new() -> Self {
        /*
            Args{
                image_1: String::new(),
                image_2: String::new(),
                output: String::new()
            }
            This was good to understand and intialize the Args struct, but we do not want empty strings for any of these values. So let's
            create something useful now
        */
        Args {
            image_1: get_nth_arg(1),
            image_2: get_nth_arg(2),
            output: get_nth_arg(3)
        }
    }
}