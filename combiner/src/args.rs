fn get_nth_arg(n: usize) -> String {
    std::env::args().nth(n).unwrap()
}

// For this project, we have declared a custom struct called Args and it's members are 2 strings (image1 and image2) along with an output of type String again.
// The struct is marked public (pub) so that it can be called in main.rs
// The members of this struct have to marked as public so they can be accessed outside of this module
pub struct Args {
    pub image_1: String,
    pub image_2: String,
    pub output: String
}