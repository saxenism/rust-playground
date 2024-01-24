fn main() {
    // Ownerships is what allows Rust to make memory safety guarentees without
    // the usage of a garbage collector, also you do not need to do manual memory management such as in C or C++
    
    // Two types of memory: Stack and Heap
    // Stack has to know the exact size of every thing in the function frame.
    // Therefore it cannot store things like string, and so it stores a pointer to a string
    // The actual string is stored in the heap memory (which is large and unorganised and slow)

    /**
     *  OWNERSHIP RULES
     * 1. Each value in Rust has a variable that is called it's owner
     * 2. There can only be one owner at a time
     * 3. When the owner goes out of scope, the value is dropped
     */

    let x = 5;
    let y = x; // A copy happens here. Since the value of x was stored in stack (fixed size)

    let s1 = String::from("Hello"); // This is a string from the heap. new and delete would have been required in Cpp
    let s2 = s1; // move happened. So basically, s1 was pointing to the string "Hello" in the stack and when we assigned s1
    // to s2, that means s2 became the pointer to "Hello" while 's1' was dropped to ensure memory safety.

    //Here is an illustration of that:
    println!("{}", s2); // this returns *Hello*
    // println("{}", s1); ❌ does not work. we get the error *value borrowed here after move*

    // In order to actually clone the value of s1's string into s2.... we need to explicity say `clone`
    let string1 = String::from("saxenism");
    let string2 = string1.clone();

    println!("{}", string1);
    println!("{}", string2);

    // When we want to use values without realling taking their ownership, because it is tedious, we use *references*
    // This is called BORROWING.
    println!("The length of {} is: {}", string1, calculate_length_via_reference(&string1)); // This ✅ works
    // println!("The length of {} is: {}", string1, calculate_length(string1)); ❌ But this would not have worked since the ownership of the string now rests with the function and we can't call string1 in the main function now 

    let mut vaakya1:String = String::from("asdfasdf");

    let r1:&String = &vaakya1;
    let r2:&String = &vaakya1;
    // let r3 = &mut vaakya1; We cannot have both immutable and mutable references of the same data in the same scope, because the immutable references do not expect the underlying data to be chaging. That assumption can be broken
    // in the presence of a mutable reference. It's absolutely fine to have multiple immutable references to the same piece of data.

    println!("{}, {}", r1, r2);
    // println!("{}, {}, {}", r1, r2, r3);

    // But it's completely fine to introduce an immutable reference here, since the scope of the mutable references r1 and r2 is finished.
    let r3 = &mut vaakya1;
    println!("r3: {}", r3);

    // let dangling_pointer: &String = reference_to_a_dropped_string();

    // Slice type references
    let mut vaakya2 = String::from("Hello World");
    let hello = &vaakya2[..5];
    let world = &vaakya2[6..];

    println!("{}, {}", hello, world);
}

fn calculate_length_via_reference(s: &String) -> usize {
    // references are immutable by default since they do NOT have ownership over the piece of data.
    // but there also exists, **MUTABLE REFERENCE**, denoted by **&mut** which allows to make changes to the referenced data.
    // However mutable references work under the restriction that there can only be 1 mutable reference for any particular piece of data in a particular scope.
    // This allows Rust to prevent data race conditions
    let length: usize = s.len();
    return length;
}

fn calculate_length(s: String) -> usize {
    let length: usize = s.len();
    return length;
}

// This is how Rust prevent dangling pointers, which is pointers pointing to invalid memory
// The string s is defined in the scope of this function and when the function gets over, the string s is dropped
// Therefore a reference to a dropped string is basically invalid memory.
// RUST PREVENTS THIS BY REFUSING TO COMPILE THIS FUNCTION WITH THE ERROR -> "expected named lifetime parameter"
// fn reference_to_a_dropped_string() -> &String {
//     let s: String = String::from("Rahul Saxena");

//     return &s;
// }
