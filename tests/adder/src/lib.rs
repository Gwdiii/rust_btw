#[derive(Debug)]
struct Rectangle {
    height: u32,
    width:  u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}",
                value
            );
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}",
                value
            );
        }

        Guess { value }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: usize) -> usize {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    // required to access assets in the encompassing module
    use super::*;

    // this will run on `$ cargo test` command
    #[test]
    fn larger_can_hold_smaller() {
        let smaller = Rectangle { height: 1, width: 5 };
        let larger  = Rectangle { height: 7, width: 8 };

        // if condition evaluates to false, `assert!` calls `panic!`
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let smaller = Rectangle { height: 1, width: 5 };
        let larger  = Rectangle { height: 7, width: 8 };

        // result is negated, as False is the expected result
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        // if both parameters express the same value, test will pass
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // if assert fails, `format!` message in second and third arguments
        // will be displayed with debug data on failure
        assert!(result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    // test will only pass if `panic!` message contains expected substring
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    // `#[should_panic]` can not be used on tests that use `Result<T, E>`
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
