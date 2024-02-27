fn variables() {
    let _apples = 5; // immutable (standard!!!)
    let mut _bananas = 5; // mutable
    let mut _guess = String::new(); // new empty string

    let x = 5;
    let y = 10;
    println!("x = {x} and y + 2 = {}", y + 2);
    // only `x = 6` doesn't compile: cannot assign twice to immutable variable `x`
    // but using let works, this is called "shadowing"
    // you can even define the scope of the shadowing with `{}`
    let x = 6;
    {
        let x = 7;
        println!("{x}"); // prints 7
    }
    println!("{x}"); // prints 6
    // shadowing is nice for somthing like this:
    let spaces = "          ";
    let spaces = spaces.len(); // if we would have used `mut` we wouldn't be able to change the type!
    println!("{spaces}"); // prints 10

    // range
    let range = 1..=100;
    println!("{:?}", range);

    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_IN_SECONDS);
}

/// Use keywords as function names
fn raw_identifiers() {
    /// Use keyword `match` as function name using `r#`
    fn r#match(needle: &str, haystack: &str) -> bool {
        // If this line has a `;` it won't return the value!
        haystack.contains(needle)
    }
    assert!(r#match("foo", "foobar"));
}

/// there are scalar and compound data types
fn data_types() {
    // ": u32" is needed here, otherwise parse does not know what to do :D
    let guess: u32 = "42".parse().expect("Not a number!"); // "42a" will panick and print error
    println!("{guess}");

    // scalar types like:
    // Integer (i8 and u8, 16, 32, 64, 126 bits, i und usize (arch))
    let truncated = 5 / 3; // Truncates toward zero to the nearest integer: so 1
    let remainder = 5 % 3; // Rest: so 2
    // floating-point (f32, f64)
    let quotient = 56.7 / 32.3; // Results in 1.755...
    println!("{quotient}, {truncated}, {remainder}");
    // bolean (bool = true | false)
    // character (char with single quotes)
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{c}, {z}, {heart_eyed_cat}");

    // compound types like:
    // tuple (fixed length, may have different types)
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_, y, _z) = tup;
    let x = tup.0;
    println!("The value of y is: {y}, the value of x is: {x}");
    // array (fixed length, same types, data pushed on the stack and not allocated on the heap)
    let a : [i32; 5] = [1, 2, 3, 4, 5]; // "; 5" specifies the length
    let b = [3; 5]; // prints [3, 3, 3, 3, 3]
    let first = a[0];
    println!("{:?}, {:?}, {first}", a, b);
    // vector (Vec<T>: no fixed length, same types, data allocated on the heap)
    let mut v1 = vec![1, 2, 3];
    print!("[");
    for i in &mut v1 { // &mut is ment for `i`, since we change it inside the loop
        print!("{}: ", &*i - 1); // de-reference `i` to be able to substract 1
        *i += 50;
        print!("{i}, ");
    }
    println!("]");
    println!("{:?}", v1); // vector has Debug but not Display
    let mut v2: Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(2);
    let _first: &i32 = &v2[0];
    let second: Option<&i32> = v2.get(1); // get() is better since we can do `match`
    match second {
        Some(second) => println!("The second element is {second}"),
        None => println!("The is no second element."),
    }
    // enum (fixed amount of different types)
    // this is used to enable a vector with different types, since a vector is allocated
    // on the heap (so we need to know the size of each element), using an enum makes
    // a variable length list with different types possible, since the actual type is `enum`
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}

/// Functions
// Open questions:
// - cannot return `str`: the size for values of type `str` cannot be known at compilation time
//   = help: the trait `Sized` is not implemented for `str`
//   = note: the return type of a function must have a statically known size
fn functions(argument1: i32, argument2: char) -> i32 {
    // everything below is a "statement": instruction that perform actions and do not return a value
    let x = argument1;
    println!("Run {x}: y: {argument2}");
    // the last is an "expression": evaluates to a result value and only works without the `;`
    x + 1
}

fn control_flow() {
    // if
    let number = 3;
    if number > 5 {
        println!("first condition was true");
    } else if number == 0 {
        println!("condition of else if was true");
    } else {
        println!("both conditions were false");
    }
    // short-form
    let number = if functions(1, 'x') == 2 { 1 } else { 0 };
    println!("Result: {}", number);

    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("again!");
        if counter == 5 {
            break 1000;
        }
    };
    println!("Loop result: {}", result);
    // loop labels!!!
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 2;
        loop {
            println!("remaining = {remaining}");
            if remaining == 0 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // while
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("{element}, ");
    }
    for number in (1..4).rev() { // rev() reverses the range, 4 is not part of the range, only 1, 2 and 3
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn convert_fahrenheit_celcius(f: f32) {
    let c = (f - 32.0) * 5.0/9.0;
    println!("{}°Fahrenheit are roughly {:.1}°Celsius", f, c);
}

fn fibonacci(position: i32) {
    let mut fib: Vec<i32> = Vec::new();
    let mut count: usize = 0;
    while count < position.try_into().unwrap() {
        if count == 0 {
            fib.push(0);
        } else if count == 1 {
            fib.push(1);
        } else {
            let pre2: &i32 = &fib[count-2];
            let pre1: &i32 = &fib[count-1];
            fib.push(pre2 + pre1);
        }
        count +=1;
    }
    println!("Fibonacchi until position {}: {:?}", position, fib);
}

fn main() {
    variables();
    raw_identifiers();
    data_types();
    println!("{}", functions(0, 'a'));
    control_flow();
    // chapter todos
    convert_fahrenheit_celcius(100.0);
    convert_fahrenheit_celcius(61.0);
    convert_fahrenheit_celcius(82.0);
    fibonacci(15);
    // skipping print_christmas_carol() because "THE TWELVE DAYS OF CHRISTMAS" is quite long
