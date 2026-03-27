fn main() {
    
    // Match Expressions

    #[derive(Debug)]
    enum Language {
        English,
        Japanese,
        Russian
    }

    let l = Language::Russian;

    match l {
        Language::English => {
            println!("English")
        },
        something_else => println!("Unsupported Language -> {:?}", something_else)
    }


    // Conditional if let expressions

    let auth_status: Option<&str> = None;
    let is_admin = false;
    let gid: Result<u8, _> = "34".parse();

    if let Some(status) = auth_status {
        println!("Auth Status : {}", status);
    } else if is_admin {
        println!("Admin");
    } else if let Ok(gid) = gid {
        println!("GID: {}", gid);
    } else {
        println!("Not recorgnized");
    }

    // While let conditional loops

    let mut stack = vec![1, 2, 4];

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // For loops
    for (ind, val) in stack.iter().enumerate() {
        println!("{} at index {}", val, ind);
    }

    // let expressions

    let x = 5;
    let (x, y, _) = (1, 2, 3);

    let point = (3,6);
    print_coord(&point);


    // Irrefutable
    let x = 6;

    // Refutable
    let x: Option<&str> = None;

    //function, let, for loops need surity so they always take input irrefutable patterns.
    // if let, while let can take in reffutable patterns, if you use irrefutable then you get a warning saying not required / useless.

    _patterns();
}

fn print_coord(&(x, y): &(i32, i32)) {
    println!("Coord : ({}, {})", x, y);
}

fn _patterns() {

    // Literal value matching
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => print!("Others")
    }

    // Named variable matching
    let x: Option<i32> = Some(7);

    match x {
        Some(5) => println!("Five"), // literal matching
        Some(y) => print!("New number : {}", y), // named var matching
        _ => println!("None")
    }

    // Multiple patterns
    let x = 2;

    match x {
        1 | 2 => println!("One or two"), // pattern matching 
        3 => println!("Three"),
        _ => println!("Something else"),
    }

    // Matching Range - Only works on numerics and chars

    match x {
        1..5 => println!("Between 1 and 4"),
        _ => println!("Outside the range 1 to 4 inclusive"),
    }

    // Deconstructing
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point {x: 1, y: 2};

    let Point {x: a, y: b} = p;
    assert_eq!(a, 1);
    assert_eq!(b, 2);

    // Deconstructing and matching named and literal values
    match p {
        Point {x: 0, y: 0} => println!("Origin"),
        Point {x: 0, y} => println!("On the x axis with y -> {}", y),
        Point {x, y: 0} => print!("On the y axis with x -> {}", x),
        _ => println!("I dont know where you are !!")
    }

    // Enum match variant ...

    // Ignoring values and patterns
    // Use _ for ignoring

    fn foo (_: i32, y: i32) {
        println!("On y -> {}", y);
    }

    // Prefix unused vars with _ to ignore compiler warning
    let _ff = 4;
    let dd = 10;

    // Important Note: _ does not move the value
    let q = String::from("value");
    // let g = q;
    let _ = q; 
    
    println!("q -> {}", q);


    // Use range syntax to ignore as well -> ..
    // Must be unabigous, this is not allowed -> .. x .. y
    match p {
        Point {x: 0 , ..} => print!("Only x is 0"),
        _ => ()
    };

    // Match Guard - Use with if and add a conditio which are too complex to show in the regular matching pattern

    let a = Some(4);
    match a {
        Some(x) if x < 5 => println!("x is less than 5"),
        _ => ()
    };

    // Bindings - Use @ to bind a value and then @ is allowed to test the condition after binding
    enum Message {
        Hello {id: i32}
    }

    // let msg = Message::Hello { id: 4 };
    let msg = Message::Hello { id: 8 };

    match msg {
        Message::Hello { id: id_var @ 3 ..=7, } => println!("Hello variant with id = {}", id_var),
        Message::Hello { id: id_var @ 1..=6,  } => println!("Hello variant with 2nd id = {}", id_var),
        Message::Hello { id } => println!("Hello variant not found in known range: {}", id)
    };
}