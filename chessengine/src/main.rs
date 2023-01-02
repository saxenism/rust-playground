/*
    /////////////////////////////////////////////
    /// Board and chess piece representations ///
    /////////////////////////////////////////////
    
    We are trying to run with each chess piece position being represented by a 64 bits binary string(will contain 64 0's or 1's).
    The idea is simple, the chess board has 64 squares and if we start numbering each of the square from the lower left corner, we can represent each piece position uniquely using a 64 bit string.
    For example:
    To represent a piece on the seventh square, the string would be like:
    0000....00010000000
    
    I know that 64 squares can be represented in a 6 bit string (2^6 = 64), but the extra bits will be helpful later on for doing different thing for each piece.
*/

fn main() {
    let position: u64 = 1;

    println!("{}", position << 63);
    println!("{}", position << 12);

    println!("Hello, world!");
}
