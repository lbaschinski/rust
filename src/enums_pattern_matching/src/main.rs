fn enums() {
    // example structs...
    struct _QuitMessage; // unit struct
    struct _MoveMessage {
        x: i32,
        y: i32,
    }
    struct _WriteMessage(String); // tuple struct
    struct _ChangeColorMessage(i32, i32, i32); // tuple struct

    // the following enum is the same as the 4 structs above!
    #[derive(Debug)]
    enum Message {
        _Quit,
        _Move { x: i32, y: i32 },
        Write(String),
        _ChangeColor(i32, i32, i32),
    }
    // but grouping everything into an enum allows defining methods!!!
    impl Message {
        fn call(&self) {
            println!("{:?}", self);
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();
}

fn pattern_matching() {
    #[derive(Debug)]
    enum UsState {
        _Alabama,
        Alaska,
        // --snip--
    }
    enum Coin {
        _Penny,
        _Nickel,
        Dime,
        Quarter(UsState),
    }
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::_Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::_Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
    let c1 = Coin::Dime;
    println!("{}", value_in_cents(c1));
    let c2 = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(c2));
}

fn options() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}, {:?}, {:?}", five, six, none)
}

fn placeholder() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // hand over parameter
        // _ => reroll(), // ignore parameter but do something
        // _ => (), // do nothing
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(_num_spaces: u8) {}
    // fn reroll() {}
}

// the same as `match` when you only want to do something for 1 case and ignore the rest
fn if_let() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else { // else branch is optional, used instead of "_" in `match`
        println!("No maximum was configured!")
    }
}

fn main() {
    enums();
    pattern_matching();
    options();
    placeholder();
    if_let();
}
