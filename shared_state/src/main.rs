use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut threads = vec![];
    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            println!("Requesting counter lock by thread: {}", i);
            let mut handle = counter.lock().unwrap();
            *handle += 1;
            println!("Add 1 to counter by thread: {}", i);
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().unwrap();
    }

    println!("New value of counter: {:?}", counter.lock());
}
