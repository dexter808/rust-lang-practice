use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut threads = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut handle = counter.lock().unwrap();
            *handle += 1;
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().unwrap();
    }

    println!("New value of counter: {:?}", counter.lock());
}
