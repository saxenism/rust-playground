fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // We know that variables in Rust are immutable by default.
    // Then why do we need the keyword *const*
    const SUBSCRIBER_COUNT: u32 = 100_000;

    // This const *variable* cannot use the *mut* keyword. So that is an extra guarentee of it's immutability
    // const mut SUBSCRIBER_COUNT -> ❌ won't work

    // Plus const variables HAVE to be **type annotated**
    // const SUBSCRIBER_COUNT: u32 = 100000; ✅
    // const SUBSCRIBER_COUNT = 100000; ❌

    // Also a const HAS to be allocated a constant value immediately.
    // const SUBSCRIBER_COUNT: u32 = 100000; ✅
    // let x = add(2, 3); ✅
    // const SUBSCRIBER_COUNT: u32 = add(2, 3); ❌

    // Compound Data types
    let tuple: (&str, i32) = ("Hello World", 100_000);

    // Value can be read by 1. Destructuring and 2. dot notation
    let (name, age) = tuple;
    let name_dot = tuple.0;
    let age_dot = tuple.1;

    let mut result = some_random_function(2, -5);
    println!("Addition result is: {}", result);

    result = some_loopy_magic(-10, 10);
    println!("Loopy magic addition: {}", result);
}

// Rust follows the snake_case convention for function names
// It is important to type annotate the input params and also the type of return value expected from the function 
fn some_random_function(a: i32, b: i32) -> i32 {
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
    // return(x + y);
    a + b // this is equivalent to return(x + y);
}

fn some_loopy_magic(a:i32, b:i32) -> i32 {
    /**
     * Some other ways to write out loops:
     * Define arrays -> let a = [1,2,3,4,5,6,7,8,9,10]
     * for element in a.iter() {
     *   sum += element
     * }
     */    
    let mut sum:i32 = 0;
    for i in a..b {
        sum += i;
    }
    return sum;
}
