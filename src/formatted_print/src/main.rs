//! Rust by Example Chapter 1

/// Formatting only for print
fn brace_formatting() {
    // There is also a macro for writing to stderr
    eprintln!("This is an error! But the program doesn't stop or fail...?");
    // There are different formatted prints
    println!("With {1} as {0}", "positions", "numbers");
    println!("{x} {y}", x="with", y="keywords");
    // With different base:
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C
    // With formatting
    println!("{number:>5}", number=1); // "    1"
    println!("{number:0>5}", number=1); // 00001
    println!("{number:0<5}", number=1); // 10000
    // a `$` is necessary when you want to use named arguments in the format specifier
    println!("{number:0>width$}", number=1, width=5);
    // combo of multiple formattings
    println!("0x{number:0>2X}", number=15);
    // set numbers of decimals with a `.`
    let pi = 3.141592;
    let decimals = 3;
    println!("Pi is roughly {pi:.decimals$}");
}

/// Formatting via the format macro to be able to reuse variable
fn macro_formatting() {
    let foo : u32 = 3735928559;
    let x = format!("{}", foo); // "3735928559"
    let hex = format!("0x{:X}", foo); // "0xDEADBEEF"
    let octal = format!("0o{:o}", foo); // "0o33653337357"
    let binary = format!("0b{:b}", foo); // "0b11011110101011011011111011101111"
    println!("Number: {} -> Binary: {} -> Octal: {}, Hexadecimal: {}", x, binary, octal, hex)
}

/// Deriving with Debug to avoid implementing Display
fn derive() {
    // Derive the `fmt::Debug` implementation for `Structure`. `Structure`
    // is a structure which contains a single `i32`.
    #[derive(Debug)]
    struct Structure(i32);

    // Put a `Structure` inside of the structure `Deep`. Make it printable
    // also.
    #[derive(Debug)]
    struct Deep(Structure);

    // `Structure` is printable thanks to `:?`!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    // There is also a pretty print here with using `#`...
    println!("{:#?}", Deep(Structure(7)));
}

/// Implement display to control how to print custom structs
fn display() {
    use std::fmt;

    struct Struct(i32);

    // To use the `{}` marker, the trait `fmt::Display` must be implemented
    // manually for the type.
    impl fmt::Display for Struct {
        // This trait requires `fmt` with this exact signature.
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Write strictly the first element into the supplied output
            // stream: `f`. Returns `fmt::Result` which indicates whether the
            // operation succeeded or failed. Note that `write!` uses syntax which
            // is very similar to `println!`.
            write!(f, "{}", self.0)
        }
    }

    struct Point2D {
        x: f64,
        y: f64,
    }
    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Customize so only `x` and `y` are denoted.
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    let x = Struct(7);
    let y = Point2D { x: 5.2, y: 6.7 };
    println!("I want this Struct to be printed: {}", x);
    println!("I want this Point to be printed: {}", y);

    // List is a special case: use `?` to continue instead of returning a Result
    struct List(Vec<i32>);
    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Extract the value using tuple indexing,
            // and create a reference to `vec`.
            let vec = &self.0;

            write!(f, "[")?;

            // Iterate over `v` in `vec` while enumerating the iteration
            // count in `count`.
            for (count, v) in vec.iter().enumerate() {
                // For every element except the first, add a comma.
                // Use the ? operator to return on errors.
                if count != 0 { write!(f, ", ")?; }
                write!(f, "{}: {}", count, v)?;
            }

            // Close the opened bracket and return a fmt::Result value.
            write!(f, "]")
        }
    }
    let v = List(vec![1, 2, 3]);
    println!("{}", v);

}

fn main() {
    brace_formatting();
    macro_formatting();
    derive();
    display();
}
