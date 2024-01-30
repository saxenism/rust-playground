use unicode_segmentation::UnicodeSegmentation;
// Strings are very complicated in Rust.
// In Rust, strings are stored as a collection of UTF-8 encoded bytes.
// ASCII only able to convert bytes to English (and vice-versa)
// So, people came up with Unicode, it has standards to encode and decode bytes to and from all the characters from the well known languages of the world.
// What's even cooler with Unicode is that it is backward compatible with ASCII (first 128 symbols are ASCII text)

// UTF-8 is a variable-width character encoding scheme for Unicode.
// By variable-width we mean that in UTF-8 a character can be represented by 1 byte, 2 byte, 3 bytes or 4 bytes.

// Rust uses UTF-8

pub fn main() {
    // Strings are stored as a collection of UTF-8 encoded bytes
    // Different ways to create strings in Rust:
    let string1 = String::new();
    let string2 = "initial contents"; // This is of type &str
    let string3 = string2.to_string();
    let string4 = String::from("initial contents");

    println!("{}", string4);

    // Since Rust strings are UTF-8 encoded, they should be able to store and interpret other languages as well
    let string5 = String::from("नमस्ते");

    println!("{}", string5);
}

pub fn append() {
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    // Not doing &s1 and &s2 because of the error: cannot add `&String` to `&String`
    // string concatenation requires an owned `String` on the left
    let s3 = s1 + &s2;

    println!("{}", s3);

    // If we wanted to do this concatenation without taking ownership, we should use the `format!` macro
    let str1 = String::from("Hello, ");
    let str2 = String::from("World!");
    
    let str3 = format!("{}{}", str1, str2);
    println!("{}", str3);
}

pub fn isolating_characters_from_string() {
    let hello = String::from("नमस्ते");

    // Since a string is a collection of bytes
    for b in hello.bytes() {
        println!("{}", b);
    }

    // The encoding of the above listed bytes as characters
    for c in hello.chars() {
        println!("{}", c);
    }

    // What we really want is the grapheme clusters and this is how we get them:
    for g in hello.graphemes(true) {
        println!("{}", g);
    }
}