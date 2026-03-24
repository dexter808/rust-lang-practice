use std::{thread, time::Duration};

fn main() {

    let v = vec![1, 2, 3, 4];

    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("At {} from spawned thread. V content -> {:?}", i, v);
            thread::sleep(Duration::from_millis(200));
        }
        "ok"
    });

    // drop(v); cannot move after move which is what we want

    for i in 1..5 {
        println!("At {} from main thread", i);
        thread::sleep(Duration::from_millis(200));
    }

    println!("Spawned thread completed with -> {:?}",handle.join().unwrap());
}
