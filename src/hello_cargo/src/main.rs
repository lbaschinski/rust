//! Doc comment that is parsed into HTML library documentation for the enclosed item...
/// Doc comment only for the following item:
fn main() {
    println!("Hello World!");
    // Normal line comment that is not parsed into HTML
    /*
    Normal block comment that is not parsed into HTML
    */
    println!("This is Rust!");
    // You can use block comments in the middle of statements!
    let x = 5 + /* 90 + */ 5;
    println!("Todays number is 10 right? x = {}", x);
}
