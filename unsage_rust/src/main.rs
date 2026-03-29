use core::slice;

unsafe extern "C" {
    fn abs(inp: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 from c -> {}", abs(-3));
    }
}