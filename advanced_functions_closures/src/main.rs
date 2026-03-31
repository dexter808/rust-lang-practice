fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f:fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn do_twice_gen<T>(f: T, arg: i32) -> i32
where T : Fn(i32) -> i32 {
    f(arg) + f(arg)
}
fn main() {
    // Function Pointers
    println!("The fn pointer value is: {}", do_twice(add_one, 5));
    
    //Closures
    println!("The Fngeneric func value is: {}", do_twice_gen(|x| x + 1, 5));
    println!("The fn pointer value is with gen: {}", do_twice_gen(add_one, 5));

    let lov_num = vec![1, 2, 3];
    let lov_str: Vec<String> = lov_num
        .iter()
        .map(|x| x.to_string())
        .collect();

    println!("lov strings: {:?}", lov_str);
    
    let los: Vec<String> = (0u32..20).map(|x| x.to_string()).collect();
    println!("Using closure and ranges: {:?}",los);


    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop
    }

    // Enum Variant tuple can be treated as function pointers
    let los: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("Using closure and ranges: {:?}",los);

}

// Continue from 7:11
fn return_closure(a: i32) -> Box<dyn Fn(i32) -> i32> {
    if a > 0 {
        move |b| a + b
    } else {
        move |b| a - b
    }
}