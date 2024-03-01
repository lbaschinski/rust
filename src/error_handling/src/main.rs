// use `panic!()` or panic-calling functions (unwrap, expect) mostly for examples, prototypes or tests
// or: if continuing would be insecure or harmful
fn unrecoverable_errors() {
    // // use panic macro to trigger an unrecoverable error
    // // and start unwinding (clean up the stack etc.)
    // panic!("crash and burn");

    // // execute code that itself triggers a panic
    // // executing "RUST_BACKTRACE=1 cargo run" shows backtrace
    // let v = vec![1, 2, 3];
    // v[99]; // buffer overread could lead to security vulnerabilities

    // the code below fails on first execution (recoverable_errors() creates these files)
    use std::fs::File;
    // unwrap() can be used if there is no special Ok() or Err() handling needed
    // It either returns the file or panics
    let greeting_file1 = File::open("hello.txt").unwrap();
    println!("File name: {:?}", greeting_file1);
    // expect() can be used for choosing the panic message
    // helps to give more context about why the operation is expected to always succeed
    let greeting_file2 = File::open("hello2.txt")
        .expect("hello2.txt should be included in this project");
    println!("File name: {:?}", greeting_file2);
}

// use `Result<T, E>` or result-producing functions (unwrap_or_else, '?') mostly for library code
// like a parser that is given malformed data ->
// when a failure is an expected possibility that the calling must decide how to handle
fn recoverable_errors() {
    use std::fs::File;
    use std::io::ErrorKind;
    let greeting_file_result = File::open("hello.txt"); // returns io::Error
    // error handling with "match"
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        }
    };
    println!("File name: {:?}", greeting_file);
    // error handling with closures (more in a later chapter...)
    let greeting_file2 = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello2.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("File name: {:?}", greeting_file2);
}

fn propagating_errors() {
    use std::fs::File;
    use std::io::{self, Read};

    // simple read that uses match
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");
        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }
    // shortened read that uses the '?' operator
    fn read_username_from_file2() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?; // see '?' usage here
        let mut username = String::new();
        username_file.read_to_string(&mut username)?; // and here
        Ok(username)
    }
    // chaining functions and '?' operator
    fn read_username_from_file3() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }
    // using standard library function
    use std::fs;
    fn read_username_from_file4() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    let mut user_name = read_username_from_file();
    println!("User name: {:?}", user_name);
    user_name = read_username_from_file2();
    println!("User name: {:?}", user_name);
    user_name = read_username_from_file3();
    println!("User name: {:?}", user_name);
    user_name = read_username_from_file4();
    println!("User name: {:?}", user_name);
}

fn question_mark_operator() {
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
    println!("{:?}", last_char_of_first_line("")); // prints "None"
    println!("{:?}", last_char_of_first_line("hello")); // prints "Some('o')"
    println!("{:?}", last_char_of_first_line("hello world")); // prints "Some('d')"
    println!("{:?}", last_char_of_first_line("hello \nworld")); // prints "Some(' ')"
}

// Use `ok()` to parse from Result to Option
fn convert_result_to_option() {
    let mut res: Result<u32, ()> = Ok(42);
    let mut opt: Option<u32> = res.ok();
    println!("{:?}", opt);
    res = Err(());
    opt = res.ok();
    println!("{:?}", opt);
}

// Use `ok_or()` or `ok_or_else()` to parse from Option to Result
// Be aware: Arguments passed to `ok_or()` are eagerly evaluated;
// if you are passing the result of a function call, it is
// recommended to use `ok_or_else()`, which is lazily evaluated.
fn convert_option_to_result() {
    let mut opt: Option<u32> = Some(42);
    let mut res: Result<u32, ()> = opt.ok_or(());
    println!("{:?}", res);
    opt = None;
    res = opt.ok_or(());
    println!("{:?}", res);
}

fn to_panic_or_not_to_panic() {
    // best practice: use `expect()` to document why a `Result` will always be `Ok()`
    use std::net::IpAddr;
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    println!("Home address: {home}");

    // functions have "contracts": behavior is only guaranteed if the inputs meet particular requirements
    // so validating input and using panic! if a contract is violated is totally fine
    // this should be documented in the API documentation for the function though!!

    // use custom types for validation to reduce invalid input posibilities
    pub struct Guess {
        // private field, to ensure that new() is used and value is validated
        value: i32,
    }
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
            Guess { value } // only create instance if validation was successful
        }
        pub fn value(&self) -> i32 { // getter function to get private field
            self.value
        }
    }
    let guess = Guess::new(50);
    // any follow-up functionality can use the Guess value without trouble
    // since it cannot be out of range
    println!("Your guess was: {}", guess.value())
}

// The main function may return any types that implement the std::process::Termination trait,
// which contains a function report that returns an ExitCode
// `Box<dyn Error>` type is a trait object, which basically means "any kind of error"
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    unrecoverable_errors();
    recoverable_errors();
    propagating_errors();
    question_mark_operator();
    convert_result_to_option();
    convert_option_to_result();
    to_panic_or_not_to_panic();

    use std::fs::File;
    // the '?' is only possible because of the changed return type of main()
    let greeting_file = File::open("hello.txt")?;
    println!("File name: {:?}", greeting_file);
    Ok(())
}
