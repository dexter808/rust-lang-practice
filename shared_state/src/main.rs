use std::sync::Mutex;
use std::thread;

fn main() {
    let m = Mutex::new(4);

    {
        let mut num = m.lock().unwrap();
        *num = 7;
    }
    println!("New value of m: {:?}", m);
}
