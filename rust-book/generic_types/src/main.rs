// Generic Types is an idea to reduce code duplication.
// Same concept as that followed in CPP
// Basically instead of taking in any particular type(string, i32, f64, etc) as an input param in a function
// Write the function logic to handle any generic input type (usually denoted by T)

// Btw, generics (generic types) do not incur any performance hit
// At compile team the code file gets flattened. So, it is all the same and no time is spent on extra eval at runtime.

fn main() {
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let number_list_2 = vec![34, 50, 25, 100, 65];

    println!("Largest number is number_list is: {}", largest_element_in_vector(number_list));
    println!("Largest number is number_list_2 is: {}", largest_element_in_vector(number_list_2));

    // Now we want to determine the largest character in a vetor of characters with the same function. How do we do that?
    let char_list = vec!['y', 'm', 'a', 'q'];

    // So, now we will re-write the largest_element_in_vector function to be able to determine the largest element irrespective of the type of vector
    println!("Largest character in char_list: {}", largest_element_in_vector(char_list));
}

// fn largest_element_in_vector(vec: Vec<i32>) -> i32 {
//     let mut largest = vec[0];

//     for number in vec {
//         if number > largest {
//             largest = number;
//         }
//     }

//     return largest;
// }

fn largest_element_in_vector<T: PartialOrd + Copy> (vec: Vec<T>) -> T {
    let mut largest = vec[0];

    for element in vec {
        // The `>` operator cannot be used on every generic type as all type cannot be compared
        // This gives the following error: binary operation `>` cannot be applied to type `T`
        // Therefore, we will restrict the type of generic types accepted by this function
        // PartialOrd -> The lt, le, gt, and ge methods of this trait can be called
        // Copy -> The generic trait can be copied. Documentation of Copy reads: Types whose values can be duplicated simply by copying bits.
        if element > largest {
            largest = element;
        }
    }

    return largest;
}

// Generics can also be used with structs
// struct Point<T> {
//     x: i32,
//     y: i32,
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }

struct Point<T, U> {
    x: T,
    y: U,
}

fn point_operations() {
    let p1 = Point{x: 5, y: 10};

    // This would not have been possible if the types were restricted to i32
    // So, to make sure both p1 and p2 are valid as long as x and y are of same type, we introduce generic traits for x and y
    let p2 = Point{x: 5.5, y: 10.9};

    // This would not have been possible if both x and y were of the same generic types.
    // So, we declared both x and y to be of different generic types to make this possible
    let p3 = Point{x: 5, y: 10.9};
}

// Generics can also be used along with enums and infact the two most popular enums: Result and Option do implement generic types already

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E), 
}

// We can define functions on struct or enums using generics 
struct Coordinate <T> {
    x: T,
    y: T,
}

impl<T> Coordinate<T> {
    fn x(&self) -> &T {
        return &self.x;
    }
}

impl Coordinate<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

// These implementation blocks imply that the `x` method would be available to the Coordinate struct of any type
// But the `y` method would only be available to the Coordinate struct of type f64.

fn coordinate_function() {
    let p1 = Coordinate{x: 5, y: 10};
    let p2 = Coordinate{x: 5.2, y: 10.9};

    println!("{}", p1.x());
    // println!("{}", p1.y()); // This doesn't work since `y` is available only to coordinate structs of type f64
    println!("{}", p2.y());
    println!("{}", p2.x());
}

