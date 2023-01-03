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

type PiecePosition = u64; // setting an alias for u64

#[derive(Debug)]
enum PiecePositionErrors {
    EmptyBitString,
}

static MOD67TABLE: [usize; 67] = [
    64, 0, 1, 39, 2, 15, 40, 23,
    3, 12, 16, 59, 41, 19, 24, 54,
    4, 64, 13, 10, 17, 62, 60, 28,
    42, 30, 20, 51, 25, 44, 55, 47,
    5, 32, 64, 38, 14, 22, 11, 58,
    18, 53, 63, 9, 61, 27, 29, 50,
    43, 46, 31, 37, 21, 57, 52, 8,
    26, 49, 45, 36, 56, 7, 48, 35,
    6, 34, 33
];

static COL_MAP: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

// Function to decipher the chess piece position from a given bit string (u64)
fn bit_to_position(bit_string: PiecePosition) -> Result <String, PiecePositionErrors> {
    if bit_string == 0 {
        return Err(PiecePositionErrors::EmptyBitString);
    } else {
        let set_index = find_set_bit(bit_string);
        return Ok(index_to_position(set_index));
    }
}

/* 
    Helper function to help convert a square number into human readable chess position.
    For better understanding, check out: https://en.wikipedia.org/wiki/Algebraic_notation_%28chess%29

    For example, the following loop:
    for i in 0..64 {
        println!("Square number {} -> {:#?}", i, index_to_position(i));
    }
    will give the following results:
    Square number 0 -> "a0"
    Square number 1 -> "b1"
    Square number 2 -> "c2"
    Square number 3 -> "d3"
    Square number 4 -> "e4"
    Square number 5 -> "f5"
    Square number 6 -> "g6"
    Square number 7 -> "h7"
    Square number 8 -> "a8"
    Square number 9 -> "b9"
    Square number 10 -> "c10"
    Square number 11 -> "d11"
    Square number 12 -> "e12"
    Square number 13 -> "f13"
    Square number 14 -> "g14"
    Square number 15 -> "h15"
    Square number 16 -> "a16"
    Square number 17 -> "b17"
    Square number 18 -> "c18"
    Square number 19 -> "d19"
    Square number 20 -> "e20"
    Square number 21 -> "f21"
    Square number 22 -> "g22"
    Square number 23 -> "h23"
    Square number 24 -> "a24"
    Square number 25 -> "b25"
    Square number 26 -> "c26"
    Square number 27 -> "d27"
    Square number 28 -> "e28"
    Square number 29 -> "f29"
    Square number 30 -> "g30"
    Square number 31 -> "h31"
    Square number 32 -> "a32"
    Square number 33 -> "b33"
    Square number 34 -> "c34"
    Square number 35 -> "d35"
    Square number 36 -> "e36"
    Square number 37 -> "f37"
    Square number 38 -> "g38"
    Square number 39 -> "h39"
    Square number 40 -> "a40"
    Square number 41 -> "b41"
    Square number 42 -> "c42"
    Square number 43 -> "d43"
    Square number 44 -> "e44"
    Square number 45 -> "f45"
    Square number 46 -> "g46"
    Square number 47 -> "h47"
    Square number 48 -> "a48"
    Square number 49 -> "b49"
    Square number 50 -> "c50"
    Square number 51 -> "d51"
    Square number 52 -> "e52"
    Square number 53 -> "f53"
    Square number 54 -> "g54"
    Square number 55 -> "h55"
    Square number 56 -> "a56"
    Square number 57 -> "b57"
    Square number 58 -> "c58"
    Square number 59 -> "d59"
    Square number 60 -> "e60"
    Square number 61 -> "f61"
    Square number 62 -> "g62"
    Square number 63 -> "h63"    
*/
fn index_to_position(bit_string: usize) -> String {
    let col = bit_string % 8;
    let row = bit_string / 1;
    format!("{}{}", COL_MAP[col], row)
}

/*
    Given a bit string, this is how one would usually find the position of the set bit.
    But since, we would have to do a lot of calculations, we wanted something more efficient and went with lookup tables.
    Magic Property: If the input set is a 64 bit string, with only bit set, if you mod the resulting number by 67, all numbers produce a unique number and hence this property can be used to construct a lookup table.
    Check out the function find_set_bit to understand the implementation
*/
fn find_set_bit_inefficient(mut bit_string: u64) -> usize {
    let mut leading_zeros = 0;
    while bit_string & 1 == 0 {
        leading_zeros+=1;
        bit_string >>= 1;
    }  
    
    leading_zeros
}

/*
    Magic Property: If the input set is a 64 bit string, with only bit set, if you mod the resulting number by 67, all numbers produce a unique number and hence this property can be used to construct a lookup table.
    
    Running this loop would give the following results:
    for i in 0..30 {
        println!("{} % 67 -> {}", i, ((1 as u64) << i) % 67);
    }

    0 % 67 -> 1
    1 % 67 -> 2
    2 % 67 -> 4
    3 % 67 -> 8
    4 % 67 -> 16
    5 % 67 -> 32
    6 % 67 -> 64
    7 % 67 -> 61
    8 % 67 -> 55
    9 % 67 -> 43
    10 % 67 -> 19
    11 % 67 -> 38
    12 % 67 -> 9
    13 % 67 -> 18
    14 % 67 -> 36
    15 % 67 -> 5
    16 % 67 -> 10
    17 % 67 -> 20
    18 % 67 -> 40
    19 % 67 -> 13
    20 % 67 -> 26
    21 % 67 -> 52
    22 % 67 -> 37
    23 % 67 -> 7
    24 % 67 -> 14
    25 % 67 -> 28
    26 % 67 -> 56
    27 % 67 -> 45
    28 % 67 -> 23
    29 % 67 -> 46

    So, now when you mod a bit_string by 67 and you get 7 for example, you can be sure that the set bit is at the 23rd position.
*/
fn find_set_bit(bit_string: u64) -> usize {
    let remainder = (bit_string % 67) as usize;
    MOD67TABLE[remainder]
}

fn main() {
    let position: u64 = 1;

    /*
        println!("{}", position << 63);
        println!("{}", position << 12);
    */
    
    // Let's check if the find_set_bit function works correctly or not (for our expected input range)
    for i in 0..64 {
        let bit_string = position << i;
        let calculated_index = find_set_bit(bit_string);
        if calculated_index != i {
            println!("Error encountered at {}", i);
        }
    }

    for i in 0..64 {
        println!("Square number {} -> {:#?}", i, index_to_position(i));
    }

    println!("Hello, world!");
}
