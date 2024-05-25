#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn _internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    _value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { _value: value }
    }
}

// Convention for unit tests: each lib file has an annotated tests module
// #[cfg(test)] tells the compiler the module below shouldn't be included in the
// regular cargo build and only for testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
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

        assert!(larger.can_hold(&smaller));
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

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    // ignores test unless explicitly requested
    // useful for tests that are expensive or take too long
    #[ignore]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    // You *can* test private functions too if you'd like
    // as long as follow Rust's privacy rules
    // The function below for example isn't marked as pub
    #[test]
    fn internal() {
        assert_eq!(4, _internal_adder(2, 2));
    }
}


/*
// Some test options:

// when needing to run the tests consecutively instead of in parallel run
$ cargo test -- --test-threads=1

// when needing to show all output even if tests succeed run
$ cargo test -- --show-output

// to run only ignored tests
cargo test -- --ignored

// to run all tests including the ignored ones
cargo test -- --include-ignored

// For all of these the -- is because the third argument is the test filter
// -- means no filter
*/