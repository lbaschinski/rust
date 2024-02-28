fn structs() {
    // struct User with 4 fields
    #[allow(dead_code)] // since fields active, username and sign_in_count are never explicitely used
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // instance of a struct
    let username = String::from("someusername123");
    let mut user1 = User {
        active: true,
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        username, // "field init shorthand" syntax (field order is not important!)
    };
    println!("Current Mail: {}", user1.email);
    user1.email = String::from("anothermail@example.com");
    println!("New Mail: {}", user1.email);

    // struct update syntax
    // moves data (username), so we cannot use `user1` afterwards anymore!
    // if we only copy fields with the "Copy" trait like `active` and
    // `sign_in_count` than `user1` would still be valid!!!
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // all other fields are the same as in user1! (must be listed last)
    };
    println!("User2: {:?}", user2);
}

fn tuple_structs() {
    struct Color(i32, i32, i32); // have struct name but no fields names
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{}: {}", black.0, origin.2);
}

fn unit_structs() {
    // unit-like structs, without any fields
    #[derive(Debug)]
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    println!("{:?}", subject);
}

fn dbg_macro() {
    // Prints and returns the value of a given expression to stderr for quick and dirty debugging
    let a = 2;
    let b = dbg!(a * 2) + 1;
    //      ^-- prints: [structs/src/main.rs:54] a * 2 = 4
    assert_eq!(b, 5);

    #[derive(Debug)]
    struct Rectangle {
        _width: u32,
        _height: u32,
    }
    let scale = 2;
    let rect1 = Rectangle {
        _width: dbg!(30 * scale), // prints: [structs/src/main.rs:65] 30 * scale = 60
        _height: 50,
    };
    dbg!(&rect1); // prints whole `&rect1` with current field values
}

/// Similar to functions, but are defined within the context of a struct/enum/trait object
fn method_syntax() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    // multiple impl blocks are also valid
    impl Rectangle {
        // this is a method in the context of the struct "Rectangle"
        fn area(&self) -> u32 {
            self.width * self.height
        }
        // methods can have the same name as fields
        fn width(&self) -> bool {
            self.width > 0
        }
        // method that writes
        fn update(&mut self) {
            self.width += 1
        }
        // method with multiple parameters
        fn can_hold(&self, rect2: &Rectangle) -> bool {
            self.width >= rect2.width && self.height >= rect2.height
        }
        // associated function! (no method, since no self parameter)
        // often used for constructors that return a new instance of a struct
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "The width of the rectangle is {} zero, it is {}.",
        if rect1.width() { "greater than" } else { "smaller than" },
        rect1.width
    );

    rect1.update();
    println!("{:?}", rect1);
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect2 = Rectangle::square(20);
    println!("This is a square: {:?}", rect2)
}

fn main() {
    structs();
    tuple_structs();
    unit_structs();
    dbg_macro();
    method_syntax();
}
