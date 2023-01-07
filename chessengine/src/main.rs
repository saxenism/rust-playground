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

use bitflags::bitflags;
use std::collections::VecDeque;

type PiecePosition = u64; // setting an alias for u64

/*
    In Rust, #[derive(Trait)] is a syntax for automatically implementing certain traits for a type. A trait is a Rust language feature that defines a set of methods that a type can implement.
    The derive attribute allows you to automatically implement one or more traits for a type, without having to write the implementation manually.

    The Debug trait is a built-in trait that allows a type to be formatted using the {:?} syntax in the format! macro and the dbg! macro. This is useful for debugging, as it allows you to print a value's internal state in a human-readable form.

    The PartialEq trait is a built-in trait that allows a type to be compared for equality using the == and != operators. This is useful for testing and other situations where you need to compare values for equality.
*/

#[derive(Debug, PartialEq, Copy, Clone)]
enum Color {
    White, 
    Black
}

#[derive(Debug, PartialEq)]
enum PieceType {
    Pawn, 
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

#[derive(Debug, PartialEq)]
struct Piece {
    position: PiecePosition,
    color: Color,
    piece_type: PieceType
}

#[derive(Debug)]
enum Square {
    Empty,
    Occupied(usize), // This usize is used to denote the piece index.
}

bitflags! {
    struct CastlingRights: u8 {
        const NONE = 0;
        const WHITEKINGSIDE = 1 << 0;
        const WHITEQUEENSIDE = 1 << 1;
        const BLACKKINGSIDE = 1 << 2;
        const BLACKQUEENSIDE = 1 << 3;
        const ALL = Self::WHITEKINGSIDE.bits | 
                    Self::WHITEQUEENSIDE.bits | 
                    Self::BLACKKINGSIDE.bits | 
                    Self::BLACKQUEENSIDE.bits;
    }
}

// Game type to own the data
// Initially this was limited to just pieces and squares, but now having studied the FEN string for chessboard representation, we gotta add a few more fields
// Full move: 1 move from white + 1 move from black
// Half move: 1 move from any player
// en_passant target square: This is a square over which a pawn has just passed while moving two squares
// castling_rights: If neither side has the ability to castle, this field uses the character "-". Otherwise, this field contains one or more letters: "K" if White can castle kingside, "Q" if White can castle queenside, "k" if Black can castle kingside, and "q" if Black can castle queenside.
struct Game {
    pieces: Vec<Piece>,
    squares: Vec<Square>,
    active_color: Color,
    castling_rights: CastlingRights,
    en_passant: Option<PiecePosition>, // Target Square. This is why it is an Option, since either you'll have a square or you won't
    halfmove_clock: usize, // The number of halfmoves since the last capture or pawn advance, used for the fifty-move rule
    fullmove_number: usize //  The number of the full moves. It starts at 1 and is incremented after Black's move.
}

impl Game {

    fn push_piece_and_square(
        &mut self,
        position: usize,
        color: Color,
        piece_type: PieceType,
        index: &mut usize
    ) {
        self.pieces.push(
            Piece{
                position: (1 as u64) << position,
                color: color,
                piece_type: piece_type
            }
        );

        self.squares.push(
            Square::Occupied(*index)
        );

        *index += 1;
    }
    
    // &mut self, since we are changing the struct `Game` itself with this function
    fn push_empty_square(&mut self) {
        self.squares.push(Square::Empty);
    }

    fn initialize() -> Game {
        let mut game = Game {
            pieces: vec![], 
            squares: vec![],
            active_color: Color::White,
            castling_rights: CastlingRights::ALL,
            en_passant: None,
            halfmove_clock: 1,
            fullmove_number: 0
        };
        let mut piece_index = 0;

        let mut color = Color::White;

        game.push_piece_and_square(
            0,
            color,
            PieceType::Rook,
            &mut piece_index
        );

        game.push_piece_and_square(
            1,
            color,
            PieceType::Knight,
            &mut piece_index
        );

        game.push_piece_and_square(
            2,
            color,
            PieceType::Bishop,
            &mut piece_index
        );

        game.push_piece_and_square(
            3,
            color,
            PieceType::Queen,
            &mut piece_index
        );

        game.push_piece_and_square(
            4,
            color,
            PieceType::King,
            &mut piece_index
        );

        game.push_piece_and_square(
            5,
            color,
            PieceType::Bishop,
            &mut piece_index
        );

        game.push_piece_and_square(
            6,
            color,
            PieceType::Knight,
            &mut piece_index
        );

        game.push_piece_and_square(
            7,
            color,
            PieceType::Rook,
            &mut piece_index
        );

        for i in 8..16 {
            game.push_piece_and_square(
                i, 
                color,
                PieceType::Pawn,
                &mut piece_index
            );
        }

        // Pushing the empty square in between the two opponents
        for i in 16..48 {
            game.push_empty_square();
        }

        color = Color::Black;

        for i in 48..56 {
            game.push_piece_and_square(
                i, 
                color,
                PieceType::Pawn,
                &mut piece_index
            );
        }

        let offset = 56;

        game.push_piece_and_square(
            0 + offset,
            color,
            PieceType::Rook,
            &mut piece_index
        );

        game.push_piece_and_square(
            1 + offset,
            color,
            PieceType::Knight,
            &mut piece_index
        );

        game.push_piece_and_square(
            2 + offset,
            color,
            PieceType::Bishop,
            &mut piece_index
        );

        game.push_piece_and_square(
            3 + offset,
            color,
            PieceType::Queen,
            &mut piece_index
        );

        game.push_piece_and_square(
            4 + offset,
            color,
            PieceType::King,
            &mut piece_index
        );

        game.push_piece_and_square(
            5 + offset,
            color,
            PieceType::Bishop,
            &mut piece_index
        );

        game.push_piece_and_square(
            6 + offset,
            color,
            PieceType::Knight,
            &mut piece_index
        );

        game.push_piece_and_square(
            7 + offset,
            color,
            PieceType::Rook,
            &mut piece_index
        );

        game
    }

    fn to_string(&self) -> String {
       let mut board = "".to_owned();
       let mut temp = "".to_owned(); 

       for (i, square) in self.squares.iter().enumerate() {
        match square {
            Square::Empty => temp.push_str(&index_to_position(i)),
            Square::Occupied(idx) => temp.push_str(&self.pieces[*idx].to_string()),
        }
        
        if(i + 1 ) % 8 == 0 {
            temp.push_str("\n");
            board.insert_str(0, &temp);
            temp.clear();
        }
       }
       // incase the if block was not activated
       board.insert_str(0, &temp);
       
       board // return the value of board
    }

    // This macro allows functions to have a non-snake case name
    #[allow(non_snake_case)]
    fn read_FEN(fen: &str) -> Game {
        let mut game = Game {
            pieces: vec![],
            squares: vec![],
            active_color: Color::White,
            castling_rights: CastlingRights::ALL,
            en_passant: None,
            halfmove_clock: 1,
            fullmove_number: 0
        };

        let (position, rest) = split_on(fen, ' ');

        let mut deque_squares = VecDeque::new();
        let mut piece_index = 0;
        let mut piece_position = 64;

        // Now we want to loop over each row present in the FEN string rep of the game.
        for row in position.splitn(8, '/' ) { // This will give you each row of the chess board.
            // Checking if we are getting the split rows from the fen_string as expected or not
            // println!("row: {}", row);

            piece_position -= 8; //Every time a row is handeled
            
            // Since we would like to construct the GAME chess board out of this FEN string row, let's call a parse function for each row
            let (pieces, squares) = parse_row(&row, piece_index, piece_position);

            // Now the immediate problem that will pop up when you try to implement the `parse_row` function is that the output of rows after `splitn` looks
            // something like this:
            /*
                row: rnbqkbnr
                row: pppppppp
                row: 8
                row: 8
                row: 8
                row: 8
                row: PPPPPPPP
                row: RNBQKBNR
            */
            // So, when we start parsing the row from the for loop, we would be doing so in a order which is opposite to the numbering convention of the chessboard
            // squares that we have been following. So, we need to introduce a data structure where we can input data from the front too (VecDeque) so that the first entry becomes the last in game.pieces and game.squares


            // Now when you get the correctly inferred pieces and squares info from `parse_row`, let's put them in the `game`'s relevant vector.
            for piece in pieces {
                game.pieces.push(piece);
            }

            for sq in squares {
                deque_squares.push_front(sq);
            }

            game.squares = Vec::from(deque_squares);

            println!("row: {}", row);
        }

        game
    }

}

    fn parse_row(
        row: &str,
        mut piece_index: usize,
        mut piece_position: usize
    ) -> (Vec<Piece>, VecDeque<Square>) {
        let mut pieces = Vec::new();
        let mut squares = VecDeque::new();

        let mut color;

        // Defining a local macro, which is just a function expanded at compile time itself.
        // It has to be local because we want it to be able to access the local variables
        // It is needed to take care of the repeated code for matching ch to chess pieces
        macro_rules! add_piece {
            ($piece_type: ident) => {
                {
                    let piece = Piece {
                        color: color,
                        position: (1 as u64) << piece_position,
                        piece_type: PieceType::$piece_type
                    };
                    let square = Square::Occupied(piece_index);
                    pieces.push(piece);
                    squares.push_front(square);
                    piece_position += 1;
                    piece_index += 1;
                }
            };
        }

        for ch in row.chars() {
            let is_upper = ch.is_ascii_uppercase();
            color = if is_upper {Color::White} else {Color::Black};
            match ch.to_ascii_lowercase() {
                'r' => add_piece!(Rook),
                'n' => add_piece!(Knight),
                'b' => add_piece!(Bishop),
                'q' => add_piece!(Queen),
                'k' => add_piece!(King),
                'p' => add_piece!(Pawn),
                num => {
                    match num.to_digit(10) { // 10 -> base 10 numbers (decimal numbers) {
                        None => panic!("Invalid Input: {}", num),
                        Some(number) => for i in 0..number {
                            squares.push_front(Square::Empty);
                            piece_position += 1;
                        }
                    }
                }
            }
        }

        (pieces, squares)
    }

    // This function will separate 1 string into 2 strings, on the separator character (omitting it)
    // Example: s = "ABCDEF", sep = 'C' -> ("AB", "DEF") // The separator character is dropped.
    fn split_on(s: &str, sep: char) -> (&str, &str) {
        // .enumerate is used to get both i and item, which is the index as well as the value of elements present in string s.
        // Simply calling for i in s.chars() would have returned the values of the subsequent chars in the string s.
        for (i, item) in s.chars().enumerate() {
            if item == sep {
                return (&s[0..i], &s[i+1..]);
            }
        }

        (&s[0..], "") // If the separator was never encountered, just return the full string and an empty string.
    }

impl Piece {
    fn to_string(&self) -> String {
        let mut result = match self.piece_type {
            PieceType::Pawn => "p ",
            PieceType::Rook => "r ",
            PieceType::Knight => "n ",
            PieceType::Bishop => "b ",
            PieceType::Queen => "q ",
            PieceType::King => "k "
        }.to_string();

        if self.color == Color::White {
            result.make_ascii_uppercase();
        }

        result
    }
}

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
    let row = bit_string / 8 + 1;
    format!("{}{} ", COL_MAP[col], row)
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
    /* 
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
    */
    let game = Game::initialize();

    let fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    let (first_row, rest) = split_on(fen_str, '/');

    println!("{}", first_row);
    println!("{}", rest);

    println!("{}", game.to_string());

    let something = Game::read_FEN(fen_str);
}
