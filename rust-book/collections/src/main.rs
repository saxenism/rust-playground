fn main() {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);

    let v2 = vec![1,2,3]; // A handy macro to do what we did with v

    let v = vec![1,2,3,4,5];

    let third = &v[2];
    println!("The third element is: {}", third);

    // The problem with the above approach for accessing certain elements is that
    // the size of a vector is unknown as opposed to array and therefore, if you try to access
    // an element that is out of bounds, your program will crash. If you donot want that happening,
    // use the `get` method which handles the *possible errors* gracefully.

    match v.get(20) {
        Some(third) => {
            println!("The third element is: {}", third);
        },
        None => {
            println!("Out of bounds, probably, lol")
        },
    }

    // Since a vector can store elements of the same type, how can store elements of multiple types into it?
    // Enums, bitch. Enums
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => {
            println!("{}", i);
        },
        _ => {
            println!("Not an integer entry");
        }
    }

    // How to iterate over a vector
    for i in &v {
        println!("{}", i);
    }

    // How to make changes over a vector? Take a mutable reference
    // And since taking the mutable and immutable reference of the same piece of data is not legal in Rust, we'll use another vector here
    for i in &mut v1 {
        *i += 50;
        println!("{}", i);
    }
}
