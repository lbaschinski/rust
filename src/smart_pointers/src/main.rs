fn box_sizing() {
    // the size of an enum is calculated from its biggest element
    // since `Cons` has a `List` again, the size calculation would be infinite
    // putting `List` in a `Box` means we don't store the list directly,
    // but a pointer to it with a fixed size (now the size for `List` can be calculated)
    // instead of storing elements inside of each other its more like next to each other
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    use List::{Cons, Nil};
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
}

fn deref_trait() {
    // dereference with `*`
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // construct a similar type to `Box` and implement `Deref` trait
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    use std::ops::Deref;
    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    let z = MyBox::new(5);
    assert_eq!(5, *z);

    fn hello(name: &str) {
        println!("Hello, {name}!");
    }
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

fn drop_trait() {
    #[derive(Debug)]
    struct CustomSmartPointer {
        data: String,
    }
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }
    {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        println!("CustomSmartPointers created: '{:?}'.", c);
        // dropping a value early with std::mem::drop
        drop(c); // c.drop() does not work, since explicit destructor calls are not allowed
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        let e = CustomSmartPointer {
            data: String::from("stuff stuff"),
        };
        println!("CustomSmartPointers created: '{:?}' and '{:?}'.", d, e);
    } // d and e go out of scope, so `Drop::drop` is called in reverse order of creation
}

fn reference_counting() {
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
    use List::{Cons, Nil};
    use std::rc::Rc;
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

fn interior_mutability() {
    // interface
    pub trait Messenger {
        fn send(&self, msg: &str); // see immutability here with `&self`
    }
    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }
    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }
        pub fn set_value(&mut self, value: usize) {
            self.value = value;
            let percentage_of_max = self.value as f64 / self.max as f64;
            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }

    // usage: might be used for testing to count messages even though we have immutable types
    use std::cell::RefCell;
    struct MyMessenger {
        // add `RefCell` here, ...
        sent_messages: RefCell<Vec<String>>,
    }
    impl MyMessenger {
        fn new() -> MyMessenger {
            MyMessenger {
                sent_messages: RefCell::new(vec![]), // ... here too, ...
            }
        }
    }
    // ... to enable the push even though `&self` is still immutable:
    impl Messenger for MyMessenger {
        fn send(&self, message: &str) { // we cannot make this `&mut self` due to trait definition
            // so we use borrow_mut() to return a smart pointer type (RefMut<T>, can only be used once!) ...
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }
    let my_messenger = MyMessenger::new();
    let mut limit_tracker = LimitTracker::new(&my_messenger, 100);
    limit_tracker.set_value(80);
    // ... and borrow() to return a smart pointer type (Ref<T>) here
    assert_eq!(my_messenger.sent_messages.borrow().len(), 1);
}

fn combined() {
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>), // combine `Rc` with `RefCell`
        Nil,
    }
    use List::{Cons, Nil};
    use std::cell::RefCell;
    use std::rc::Rc;
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil))); // use `Rc::clone()` again
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    // use automatic dereferencing via calling `borrow_mut()` to dereference the `Rc<T>`
    // to the inner `RefCell<T>` (returns a RefMut<T> smart pointer)
    // then dereference via `*` to change the inner value
    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

fn main() {
    box_sizing();
    deref_trait();
    drop_trait();
    reference_counting(); // multiple owners
    interior_mutability(); // mutable data
    combined(); // multiple owners of mutable data
}
