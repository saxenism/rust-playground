// Dangling references -> References that point to invalid data and Rust DOES NOT like dangling references.

use std::ops::Deref;

fn main() {
    // This would not compile. Why?
    // Read the reason at line 14.

    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    // }

    // In line 16, r is a dangling reference to x because x has been dropped from memory ever since it's scope ended.
    // Rust compiler can stop the compilation bc it knows it's a dangling reference at compile time because of it's borrow checker.
    // println!("r: {}", r);

    // What will compile? Well, the code where both r and x have the same lifetimes
    // In the above cases, the compiler was able to figure out the obvious mistakes in the lifetimes, but there would be cases where we would have to help the compiler with generic lifetime annotation
    let r;
    let x = 5;
    r = &x;
    println!("r: {}", r);
    println!("x: {}", x);

    example_generic_lifetime_annotations();
    example_generic_lifetime_annotations_3();

    split_sentence_does_not_compiles();
}

fn example_generic_lifetime_annotations() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());

    // Now once we have specified the x and y and result have the same lifetime: 'a
    // That means that: lifetime_result = min(lifetime_x, lifetime_y);
    // That means the only thing that the compiler needs to check with the borrow-checker is that when we are calling the `result` variable here are the lesser lifetime valid string still in scope or not. That's it. 
    println!("The longest string is: {}", result);

    // If you want to see how the lifetime affect this function, read example_generic_lifetime_annotations_2
}

/*
    fn example_generic_lifetime_annotations_2() {
        let string1 = String::from("abcd");
        let result;
        {
            let string2 = String::from("efgh");
            result = longest(string1.as_str(), string2.as_str());
        }

        println!("The longest string is {}", result);
    }
*/

fn example_generic_lifetime_annotations_3() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("efgh");
        result = longest_always_return_x(string1.as_str(), string2.as_str());
    }

    println!("The longest string is {}", result);
}

// The following function definition fails to compile with the following error:
// missing lifetime specifier this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
// which makes sense if you think that x and y themselves could have different lifetimes... so there is an ambiguity in the return value of this function's lifetime.
/*
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}
*/

// Writing the function again with generic lifetime annotations, which does not modify the lifetimes of anyone but rather explain the relation between two contesting lifetimes
fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}
// Remember that generic lifetime annotations doesnot change the lifetimes of different references, but rather establish a relationship.
// Since we say here that the resulting str has a lifetime equal to x and y. That means it will be equal to a value that applies to both x and y.
// What would be that value if the lifetimes of both x and y are different? The smaller of the two values would be the accepted value for the resultant string, as for that particular value x, y, and the resultant string are all valid and in the memory scope.

// Since this function always returns the string x as the largest one, and we do not really use y, we removed any lifetime annotation from y without any issues
// It will be interesting to see the implication of this function in example_generic_lifetime_annotations_3
// Why? Because you'll see that the error the crept in example_generic_lifetime_annotations_2 has gone away now.
// When you look at it, it kinda feels like that the println!() statement at line 64 should not compile but it does, because now...
// the compiler doesn't give 2 flying fucks about the lifetime of string y and how it affects the lifetime of the result string.
fn longest_always_return_x<'a> (x: &'a str, y: &str) -> &'a str {
    return x;
}

// The lifetime of the return value from a function has to be tied with some of the input parameter(s).
// That's because we cannot return a reference of something that was created inside of the function, because that shit drops out of memory as soon as the function stops executing
// Example: The below function does not compile
// You error out at the return statement with this: cannot return value referencing local variable `result`
//                                                  returns a value referencing data owned by the current function
/* 
    fn longest_expt<'a> (x: &str, y: &str) -> &'a str {
        let result = String::from("really long string");
        return result.as_str();
    }
*/

// How you could have mitigated that is instead of returning &str, you return String
// Example: The below function compiles.
// Why? Because String is an owned data type and it transfer ownership wherever it is sent by the current owner.
fn lonest_expt<'a> (x: &'a str, y: &'a str) -> String {
    let result = String::from("really really long string");
    return result;
}
// First off, the lifetime here is irrelevant, compiles without it.
// Secondly, When result: String is declared it is owned by the function 
// And when it is returned to whoever the fuck calls it, the result: String is then owned by them.

// When using non-owned data in structs, we need to use lifetime annotations:
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// The above declaration basically means that **OUR STRUCT CANNOT OUTLIVE THE LIFETIME OF THE PART &STR THAT IS PASSED TO IT**. Simple.
fn split_sentence_compiles() {
    let novel = String::from("Call me Rahul. Rahul Saxena. Naam to suna hi hoga. Ahahahahahahahahaha.");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence
    };
    println!("{}", i.part);
}

fn split_sentence_does_not_compiles() {
    let novel = String::from("Call me Rahul. Rahul Saxena. Naam to suna hi hoga. Ahahahahahahahahaha.");
    {
        let mut i = ImportantExcerpt{part: "asdf"};
        {
            let first_sentence = novel.split('.').next().unwrap();
            i.part = first_sentence;
        }
        println!("{:#?}", i);
    }
    // println!("{}", i.part);
}

/*
//////////////////////////////////////////////////
/// LIFETIME ELISION RULES
/// HOW COMPILERS AUTOMATE THIS LIFETIME STUFF
/////////////////////////////////////////////////

1. Each parameter that is a reference gets its own lifetime parameter
2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
3. If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters

    Rule 3 only applies to methods (in impl blocks)

//////////////////////////////////////////
/// STATIC TIMELINES
//////////////////////////////////////////

Static lifetime means that the reference can live for the duration of the program.
All string literals have a static lifetime, since string literals are stored in a program's binary

*/