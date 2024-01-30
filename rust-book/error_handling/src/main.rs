use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("Hello, world!");

    // If your program fails in a way that is not recoverable or you can't handle the error gracefully,
    // consider using the panic macro
    // panic!("Crash and burn");

    // The function File::open returns a Result enum.
    // If the file existed and opened successfully, then you get some resultant value in Ok
    // Else you get some Error value in Err. 
    let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:#?}", error),
    // };

    // Suppose we want to handle this a little more gracefully and create a new file if the given file does not exist
    // We match the errorkind to NotFound and if it matches, we try to create the non-existent file
    // But since an exception could come up even when creating the file, we created another match statement to deal with the error
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:#?}", other_error)
    //         },
    //     }
    //};

    // Let's talk about some useful function on the Result enum
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:#?}", error),
    // };
    // The above code is equivalent to:
    let file = File::open("hello.txt").expect("Failed to open hello.txt");
    // The `?` operator is very similar to this `expect`/`unwrap` in the sense that if the function call
    // goes through as expected then there is no issues, but incases there is an error, the program exits early

    // The use of expect and unwrap etc is not promoted in serious codebases as proper error-handling along with
    // proper error propagation is expected.
}
