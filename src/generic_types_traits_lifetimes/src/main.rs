// generic types
fn removing_duplication_with_functions() {
    // function that de-duplicates functionality
    fn largest(list: &[i32]) -> &i32 {
        let mut largest = &list[0];
        for number in list {
            if number > largest {
                largest = number;
            }
        }
        largest
    }
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
}

fn removing_duplication_with_generic_types() {
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn generics_in_structs_enums_methods() {
    // structs
    #[derive(Debug)]
    struct Point<T> {
        _x: T, // T must be the same for x and y
        _y: T,
    }
    let integer = Point { _x: 5, _y: 10 };
    let float = Point { _x: 1.0, _y: 4.0 };
    println!("Two points: {:?}, {:?}", integer, float);

    #[derive(Debug)]
    struct Point2<T, U> {
        _x: T,
        _y: U,
    }
    let both_integer = Point2 { _x: 5, _y: 10 };
    let both_float = Point2 { _x: 1.0, _y: 4.0 };
    let integer_and_float = Point2 { _x: 5, _y: 4.0 };
    println!("Three points: {:?}, {:?}, {:?}", both_integer, both_float, integer_and_float);

    // enums
    #[derive(Debug)]
    pub enum MyResult<T, E> {
        MyOk(T),
        MyErr(E),
    }
    let my_result: MyResult<u32, ()> = MyResult::MyOk(42);
    let my_error: MyResult<u32, String> = MyResult::MyErr("error!".to_string());
    println!("My ok enum: {:?}", my_result);
    println!("My err enum: {:?}", my_error);

    // methods
    // method for generic Point
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self._x
        }
    }
    println!("p.x = {}", integer.x());
    println!("p.x = {}", float.x());
    // method for a Point that has only float values (f32)
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self._x.powi(2) + self._y.powi(2)).sqrt()
        }
    }
    // Using the function on an `integer` Point fails with "method not found in `Point<{integer}>`"
    // println!("Distance from origin: {}", integer.distance_from_origin());
    println!("Distance from origin: {}", float.distance_from_origin());
}

// traits
fn traits() {
    use std::fmt;
    // defining a trait
    pub trait Summary {
        // to be implemented by each struct on their own
        fn summarize1(&self) -> String;
        fn summarize_author(&self) -> String;
        // provide a default implementation
        fn summarize2(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }
    #[derive(fmt::Debug)]
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    // implementing a Trait on a Type
    impl Summary for NewsArticle {
        fn summarize1(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
    }
    #[derive(fmt::Debug)]
    pub struct Tweet {
        pub username: String,
        pub content: String,
    }
    // implementing a Trait on a Type
    impl Summary for Tweet {
        fn summarize1(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
    };
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("1 new tweet: {}", tweet.summarize1());
    println!("New article available! {}", article.summarize2());

    // Traits as Parameters
    // `item` can be of any type that implements the Summary and Debug Trait!
    // This is syntax sugar for "trait bound": pub fn notify<T: Summary + Debug>(item: &T)
    pub fn notify(item: &(impl Summary + fmt::Debug)) {
        println!("Breaking news! {} {}", item.summarize1(), item.summarize2());
    }
    // Both forms are sometimes better sometimes worse for different use-cases:
    // - if we have multiple parameters that implement Summary
    //      pub fn notify(item1: &impl Summary, item2: &impl Summary) // enables different types
    //      pub fn notify<T: Summary>(item1: &T, item2: &T)           // enforces the same type!
    notify(&tweet);
    notify(&article);

    // Clearer Trait Bounds with where Clauses!!
    // Original: fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    fn _some_function<T, U>(_t: &T, _u: &U) -> i32
    where
        T: fmt::Display + Clone,
        U: Clone + fmt::Debug,
    {
        42
    }

    use std::fmt::Display;
    struct Pair<T> {
        x: T,
        y: T,
    }
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
    let _x = Pair::new(&tweet, &tweet);
    let y = Pair::new(2, 3);
    // _x.cmp_display(); // cannot work since Tweet does not implement PartialOrd!
    y.cmp_display();

    // blanket implementations:
    // implement a trait conditionally for any type that implements another trait
    /*
    impl<T: Display> ToString for T {
        // --snip--
    }
    */
}

// lifetimes
fn lifetimes() {
    // borrow checker checks lifetimes, are defined as `'<name>`
    // `&x` has no lifetime parameter, `&'a x` and `&'a mut x` have an explicit lifetime `'a`
    // lifetime annotations don't change how long any reference lives
    // they tell how generic lifetime parameters of multiple references relate to each other
    // lifetime annotations are part of the contract of the function (like the types in the signature)
    fn longest<'a>(x: &'a str, y: &str) -> &'a str { // result has the same output lifetime than x' input lifetime
        println!("y will never be the longest: {}", y);
        x // ys input lifetime does not matter, since it is not used for the result
    }
    let string1 = String::from("abcd");
    let result;
    {
        // since we specify lifetimes in `longest()`, it doesn't matter that string2 does not live long enough
        let string2 = "xyz";
        result = longest(string1.as_str(), string2);
    } // result is valid even though string2 does not live after this
    // this would not work if we don't use lifetime annotation
    println!("The longest string is {}", result);

    // lifetime in structs
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        _part: &'a str, // needs lifetime since this string slice is a reference!
    }
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        _part: first_sentence,
    };
    println!("{:?}", i);

    // lifetime elision rules:
    // 1. each parameter gets its own lifetime
    // 2. if there is exactly one input lifetime, the output lifetimes are the same as the input one
    // 3. if there are multiple input lifetimes, but one of them is `&self` or `&mut self` (methods),
    //      the output lifetimes are the same as the `self` input lifetime

    // lifetime in method definitions
    impl<'a> ImportantExcerpt<'a> {
        fn _level(&self) -> i32 {
            3 // unrelated, does not need lifetime
        }
        fn _part(&'a self) -> &'a str {
            self._part // related to a reference, we could add lifetime or ...
        }
        fn _part_inferred(&self) -> &str {
            self._part // inferred through rule 1 and 2
        }
        fn _inferred_announce_and_return_part(&self, announcement: &str) -> &str { // let it be inferred through rule 1 and 3
            println!("Attention please: {}", announcement);
            self._part
        }
    }

    // static lifetime: (put directly into binary...)
    // can live for the entire duration of the program!
    // (all string literals have the static lifetime...)
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);
}

fn all_together() {
    use std::fmt;

    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T, // any type that implements Display (lifetime is not important, since we don't use for result)
    ) -> &'a str // result has x and ys lifetime, since both could be part of the result
    where
        T: fmt::Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    pub struct Announcement {
        pub content: String,
    }
    impl fmt::Display for Announcement {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.content)
        }
    }
    let announcement = Announcement { content: "content".to_string() };
    println!("{}", longest_with_an_announcement("xyz", "y", announcement));
}

fn main() {
    removing_duplication_with_functions();
    removing_duplication_with_generic_types();
    generics_in_structs_enums_methods();
    traits();
    lifetimes();
    all_together();
}
