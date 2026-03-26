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

}

fn print_coord(&(x, y): &(i32, i32)) {
    println!("Coord : ({}, {})", x, y);
}
