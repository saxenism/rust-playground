// This is how you define a struct in Rust.
// Pretty much like any other language
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)] // We'll learn more about traits in chapter 10
struct Rectangle {
    width: u32,
    height: u32
}

// This block will house the methods associated with our structs
impl Rectangle {
    // The first argument in such impl functions is always self, which is the struct instance
    // on which the method is called. Also possible to make mutable references.
    fn rectangle_impl_area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Btw struts allow for multiple implementation blocks
impl Rectangle {
    // This is an associated function and NOT a method, we don't get passed in the self argument
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    
    // A User instantiation can be created and the properties can be populated in any order
    let mut user1 = User {
        email: String::from("saxenism@gmail.com"),
        username: String::from("saxenism"),
        active: true,
        sign_in_count: 1
    };

    // The properties of a struct object can be accessed via dot notation
    
    /*
    let name = user1.username;
    println!("This should fail because of *move*: {}", user1.username);
    */
    
    user1.username = String::from("YaamiDancho");
    println!("The username should have changed to: {}", user1.username);

    let user2 = build_user(
        String::from("captain@blackbulls.com"),
        String::from("Yami Sukehiro")
    );

    println!("The user2's name & email is: {} & {}", user2.email, user2.username);

    // This is how we create a user3 using a few properties of user2
    let user3 = User {
        username: String::from("Asta"),
        email: String::from("zeromana@blackbulls.com"),
        ..user2
    };

    println!("Checking properties of user3: {}, {}", user3. email, user3.sign_in_count);

    // Tuple structs can be created without named fields
    // For example: 
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectange is {} square pixels",
        area(width1, height1)
    );

    // This should work, since it is not a dynamic data type.
    println!("{}", width1);

    let rect = (30, 50);

    println!(
        "The area of the rectange is {} square pixels.",
        better_area_function(rect)
    );

    let rectangle: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectange is {} square pixels.",
        better_yet_area_function(&rectangle)
    );

    // This should not work, since value of rectange moved to the function `better_yet_area_function`
    // println!("{}", rectangle.width);
    // Had we wanted to avoid this, we should have kept the input arg of better_yet_area_function as &Rectangle

    // Since the Rust compiler does not know how to print out the struct Rectangle,
    // we will add the Debug trait to the struct.
    println!("rectangle: {:#?}", rectangle);

    // Using the impl methods on Rectangle structs
    println!("The area of the rectange is: {}", rectangle.rectangle_impl_area());

    let rect1 = Rectangle{
        width: 20,
        height: 40
    };
    
    let rect2 = Rectangle {
        width: 40,
        height: 60
    };

    let rect3 = Rectangle::square(25);

    println!("Can rectangle hold rect1: {}", rectangle.can_hold(&rect1));
    println!("Can rectangle hold rect2: {}", rectangle.can_hold(&rect2));

    println!("What is the dimensions of Rectangle3: {:#?}", rect3);

}

// This function is used to calculate the area of a rectangle but is defined separately.
// We can tightly couple this function with instances of the Rectangle struct via the `impl` block.
// Let's see how that goes
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn better_area_function(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn better_yet_area_function(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

fn build_user(email: String, username: String) -> User {
    // User {
    //     email: email,
    //     username: username,
    //     active: true,
    //     sign_in_count: 1,
    // } -> This works obviously, but we can simplify this 

    // This is called the field init shorthand syntax
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}