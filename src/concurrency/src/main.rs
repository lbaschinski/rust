fn threads() {
    use std::thread;
    use std::time::Duration;
    // returns a `JoinHandle`
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // without this the thread would have been killed as soon as
    // the program completes and the output would have been shorter
    handle.join().unwrap();

    // use `move` in the closure since `v` is borrowed otherwise and
    // we cannot be sure that `v` lives long enough
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // drop(v); for example would break the code if we don't use `move`,
    // since it could be executed before the `println` and then `v` would be gone.
    // It still fails with the `move` though, since we moved `v`...
    handle.join().unwrap();
}

fn message_passing() {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;
    // create a channel with mpsc (multiple producer, single consumer)
    // returns tuple of (trasmitter, receiver)
    let (tx, rx) = mpsc::channel();
    // create another producer
    let tx2 = tx.clone();
    // let the transmitter send one string from within a spawned thread
    thread::spawn(move || {
        let val = String::from("hi - this is a oneliner");
        // transfer ownership of `val` to receiver via `send()`
        tx.send(val).unwrap();
        // cannot use `val` here since ownership was tranferred
    });
    // using blocking `recv()` here, also non-blocking `try_recv()` exists
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // send multiple messages from another producer while receiver is waiting
    thread::spawn(move || {
        let vals = vec![
            String::from("- multiliner -"),
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // don't use `recv`, treat `rx` as an iterator
    // there is no pausing or delaying code below!
    // iteration ends when the channel is closed (end of thread above)
    for received in rx {
        println!("Got: {}", received);
    }
}

fn shared_state() {
    use std::sync::Mutex;
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap(); // gets the lock to change the number
        *num = 6;
    } // releases the lock automatically since it is a smart pointer
    println!("m = {:?}", m);

    // sharing a Mutex<T> between multiple threads (only possible with multiple ownership)
    use std::sync::Arc; // Atomically Reference Counted (Rc for concurrent situations)
    use std::thread;
    let counter = Arc::new(Mutex::new(0)); // same API as Rc...
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter); // ... so we only need to replace 3 lines
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}

fn main() {
    threads();
    message_passing();
    shared_state();
}
