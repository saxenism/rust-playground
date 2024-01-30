// Hashmaps allow you to store key-value pairs
use std::collections::HashMap;

pub fn main() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    // This would fail since we are NOT passing the string reference into the `insert` function:
    // println!("{}", blue);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(s) => {
            println!("The score is: {}", s);
        },
        None => {
            println!("No such team exists");
        }
    }

    // We can also iterate over all the values in a hashmap (not possible in Solidity)
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

}

pub fn create_mapping_of_frequency_of_words() {
    let text = "hello world wonderful world";

    // What we need to do is write a function that outputs the frequency of each word in the `text`
    let mut map: HashMap<&str, isize> = HashMap::new();

    for word in text.split_whitespace() {
        // The function entry returns the value at the `key` which could be an existing value or be empty
        // Now what or_insert does here is that if it was empty, gives it a value of 0 and if there was a value already, it does not do anything
        // And it also happens to return a mutable reference to the value of the key
        // So, we de-reference it and increase the value to match the frequency.
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
}