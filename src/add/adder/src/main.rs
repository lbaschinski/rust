use add_one;
use add_two;

fn main() {
    let num1 = 10;
    let num2 = 10;
    println!("Hello, world! {num1} plus one is {}!", add_one::add_one(num1));
    println!("Hello, world! {num2} plus one is {}!", add_two::add_two(num2));
}
