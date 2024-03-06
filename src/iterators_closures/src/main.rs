fn closures() {
    #[derive(Debug, PartialEq, Copy, Clone)]
    enum ShirtColor {
        Red,
        Blue,
    }

    struct Inventory {
        shirts: Vec<ShirtColor>,
    }

    impl Inventory {
        fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
            // this is a closure:
            user_preference.unwrap_or_else(|| self.most_stocked())
        }

        fn most_stocked(&self) -> ShirtColor {
            let mut num_red = 0;
            let mut num_blue = 0;

            for color in &self.shirts {
                match color {
                    ShirtColor::Red => num_red += 1,
                    ShirtColor::Blue => num_blue += 1,
                }
            }
            if num_red > num_blue {
                ShirtColor::Red
            } else {
                ShirtColor::Blue
            }
        }
    }

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // closure: borrowing immutably (implements Fn trait and can be called multiple times)
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // closure: borrowing mutably (implements FnMut trait and can be called multiple times)
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    // closure: takes ownership (implements FnOnce trait and can be called once!!!)
    use std::thread;
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    // force taking ownership through `move`
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // FnMut example with `sort_by_key` (does not mutate anything, but calls closure multiple times)
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        _height: u32,
    }
    let mut list = [
        Rectangle { width: 10, _height: 1 },
        Rectangle { width: 3, _height: 5 },
        Rectangle { width: 7, _height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}

fn iterators() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter(); // create iterator, lazy evaluation
    // Iterators implement a trait named Iterator and have the next() method
    assert_eq!(v1_iter.next(), Some(&1)); // for next() to work, the iterator must be mutable
    for val in v1_iter { // go through left over elements of iterator
        println!("Got: {}", val);
    }
    // next() returns None if iterator is empty
    let v2: Vec<i32> = vec![];
    let mut v2_iter = v2.iter();
    assert_eq!(v2_iter.next(), None);

    // consuming methods
    let v3 = vec![1, 2, 3, 4];
    let v3_iter = v3.iter();
    let total: i32 = v3_iter.sum(); // sum() is an "consuming adaptor" and calls next() internally
    assert_eq!(total, 10);

    // iterator producing methods
    let v4: Vec<i32> = vec![1, 2, 3];
    // map consumes the iterator and creates another one, which is consumed through the collect method
    let v5: Vec<_> = v4.iter().map(|x| x + 1).collect();
    assert_eq!(v5, vec![2, 3, 4]);
}

fn capture_environment() {
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }
    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        // `filter()` takes a closure that gets an item from the iterator and returns a bool
        // if the bool is true, the item will be included in the iteration produced by filter
        // `into_iter()` takes ownership of the vector `shoes`
        // `collect()` gathers the values returned by the adapted iterator into a new vector which is returned
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}

fn main() {
    closures();
    iterators();
    capture_environment();
}
