//! Ownership rules
//! 1. Each value in Rust has an owner.
//! 2. There can only be one owner at a time.
//! 3. When the owner goes out of scope, the value will be dropped.

fn move_clone() {
    let x = 5;
    let _y = x; // this is a copy to the stack and is also 5 (cheap)
    // Trait: Copy (all interger types, boolean, all floating-point types, char, Tuples with only types that implement Copy)

    let s1 = String::from("hello"); // s1 holds ptr, len and capacity
    let s2 = s1; // this is no simple copy of the String itself, but a "move" of the pointer to "hello" on the heap (cheap)
    // to ensure we do not free twice (double free error), s1 is no longer valid after being "moved" into s2
    // println!("{}, world!", s1); // does not compile!!!
    // Trait: Drop

    let s3 = s2.clone();
    println!("{}, {}!", s2, s3); // does compile since we did clone here (expensive)

    // Ownership and Functions:

    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing
      // memory is freed.

    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.

    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    // cannot use s here, since it would throw a compile-time error
    // println!("{s}");
    // Resolve: we could either let `takes_ownership` return s as a result,
    // use `clone()` or use references

    let x = 5;                      // x comes into scope
    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
    // can use x here, since we copied it
    println!("{x}");

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

/// References are ptr that point to (ptr, len, capacity)
fn references() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1); // hand over a reference instead of moving
    println!("The length of '{}' is {}.", s1, len);
    let len2 = calculate_length_mut(&mut s1);
    println!("The length of '{}' is {}.", s1, len2);

    fn calculate_length(s: &String) -> usize { // s is a reference to a String
        // s.push_str("..."); // since s is borrowed, we cannot change it
        s.len()
    }

    fn calculate_length_mut(s: &mut String) -> usize { // s is a mutable reference to a String
        s.push_str("..."); // since s is mutable now, we can change it
        s.len()
    } // s goes out of scope here, but remember that we changed the content of s1

    // there can always be only 1 mutable reference or many immutable once together
    let r1 = &s1; // since they are only readable they cannot affect anyone else
    let r2 = &s1;
    println!("{}, {}", r1, r2);
    let r3 = &mut s1; // as long as r1 and r2 are not used anymore after this it's fine
    // let r4 = &mut s1; // this is not compiling
    println!("{}", r3);
}

/// Reference to a part of a string. Therefore always valid.
fn slice() {
    let s = String::from("hello world");

    let _hello = &s[..5]; // same as [0..5]
    let _world = &s[6..]; // same as [6..11] where 11 is s.len()
    let _hello_world = &s[..]; // same as [0..11], so the whole string

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..] // same as `&s`, I don't know why the example wants to use the slice here
    }

    let my_string = String::from("hello world");
    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let _word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = first_word(&my_string);

    let my_string_literal = "hello world";
    // `first_word` works on slices of string literals, whether partial or whole
    let _word = first_word(&my_string_literal[0..6]);
    let _word = first_word(&my_string_literal[..]);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word(my_string_literal);
    println!("the first word is: {}", word);

    // other slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    move_clone();
    references();
    slice();
}
