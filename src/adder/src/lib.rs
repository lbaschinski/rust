pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Debug)]
struct Rectangle {
    _width: u32,
    _height: u32,
}
impl Rectangle {
    fn _can_hold(&self, other: &Rectangle) -> bool {
        self._width > other._width && self._height > other._height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

// run `cargo test -p <package>` to run _all_ tests in this package or
// `cargo test <test_name>` to run _one_ specifically or
// `cargo test <word>` to run all tests that _include_ `word` or
// `cargo test -- --ignored` to run _only_ the ignored tests or
// `cargo test -- --include-ignored` to run _all including ignored_ tests
#[cfg(test)]
mod tests {
    use super::*;

    // add tests
    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
    // doesn't make sense, just for `assert_ne` testing purposes
    #[test]
    fn it_adds_two_2() {
        let result = add_two(2);
        assert_ne!(result, 5);
    }
    // you can also test private functions
    #[test]
    fn adds_internally() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
    // use should_panic to let tests that test panic behavior succeed
    // run `cargo test -- --show-output` to display output like the panic message below
    #[test]
    #[should_panic]
    fn it_fails() {
        panic!("Make this test fail");
    }
    // Rectabgle tests
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            _width: 8,
            _height: 7,
        };
        let smaller = Rectangle {
            _width: 5,
            _height: 1,
        };
        assert!(larger._can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            _width: 8,
            _height: 7,
        };
        let smaller = Rectangle {
            _width: 5,
            _height: 1,
        };
        assert!(!smaller._can_hold(&larger));
    }
    // greeting tests
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
    #[test]
    #[should_panic(expected = "Greeting did not contain name")] // add expected str to verify panic message
    fn greeting_contains_name_2() {
        let result = greeting("Carol");
        assert!(
            result.contains("Robert"),
            "Greeting did not contain name, value was `{}`", // custom panic message
            result
        );
    }
    // using Result<T, E>: enables usage of `?`, so that you can test if any operation inbetween returns Err
    #[test]
    fn it_works() -> Result<(), String> {
        if add_two(2) == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    // but you can also not use `?` and convert a Result<T, E> back to an panic
    #[test]
    #[should_panic]
    fn it_works_2() {
        let sum = add_two(2);
        let result;
        if sum == 4 {
            result = Ok(())
        } else {
            result = Err(String::from("two plus two does not equal four"))
        }
        assert!(result.is_err())
    }
    // exclude tests on purpose
    #[test]
    #[ignore]
    fn expensive_test() {
        panic!("I should be excluded!")
    }
}
