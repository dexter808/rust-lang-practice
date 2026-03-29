static mut COUNTER: u32 = 0; 

fn main() {

    unsafe {
        COUNTER += 1;
    }

    unsafe {
        // println!("Value is {}", COUNTER); This is not allowed anymore from 2024
        let v = COUNTER; // Copy then use
        println!("New value is -> {}", v);
    }
}