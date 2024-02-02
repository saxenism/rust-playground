#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height 
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_ne!(result, 23);
    }

    #[test]
    #[should_panic]
    fn failing_test() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller), "Wow!! This is a wonderful error message");
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!(smaller.can_hold(&larger)));
    }


    // Returing a Result type
    #[test]
    fn it_works_again() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("The addition is wrong"))
        }
    }

}