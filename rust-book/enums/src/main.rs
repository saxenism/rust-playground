// In Rust, enums are incredibly powerful and similar in its implementation to functional languages (which I have zero idea about :) )

// Enums -> allows you to enumerate a list of variants.
// Example use case -> To easily identify which kind of IP Address is being used in code. Ver4 or 6.

#[derive(Debug)]
enum IpAddrKind {
    v4,
    v6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String
}

#[derive(Debug)]
enum IPAddressWithKind {
    v4(String),
    v6(String)
}

#[derive(Debug)]
enum IpAddrKindWithBits {
    v4(u8, u8, u8, u8),
    v6
}

// An enum can serve as a handy grouping of different types of structs. (instead of declaring a lot of different structs, you can use 1 enum)
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

// Just like a struct, an enum can have impl blocks as well
impl Message {
    fn some_function() {
        println!("Hello Message from Message Enum");
    }
}

fn main() {
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;

    // This is cool, but we can make this even more compact by putting data
    // directly inside the enums
    // let localhost = IpAddr {
    //     kind: IpAddrKind::v4,
    //     address: String::from("127.0.0.1"),
    // };

    // println!("{:#?}", localhost);

    // let localhost = IPAddressWithKind::v4(String::from("127.0.0.1"));
    // println!("{:#?}", localhost);

    // Again very cool stuff, but we can also store different kinds of data in enum types
    let localhost = IpAddrKindWithBits::v4(127,0,0,1);
    println!("{:#?}", localhost);
}

// Similarly there exists an Option enum with a Some and None values. It can be used along with the 
// match expressions to handle cases of null values in Rust.
