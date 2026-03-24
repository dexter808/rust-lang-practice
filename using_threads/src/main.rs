use std::{thread, time::Duration};

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("At {} from spawned thread", i);
            thread::sleep(Duration::from_millis(200));
        }
    });

    for i in 1..5 {
        println!("At {} from main thread", i);
        thread::sleep(Duration::from_millis(200));
    }
}
