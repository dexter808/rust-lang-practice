fn main() {
    let mut num = 5;

    let r1 = &num as *const i32; // immutable raw pointer
    let r2 = &mut num as *mut i32; // mutable raw i32 pointer

    unsafe {
        println!("r1 : {}", *r1);
        println!("r2 : {}", *r2);
    } 
}
