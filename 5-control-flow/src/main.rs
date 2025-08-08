fn main() {
    exercise1();
    exercise2();
    exercise3();
}

fn exercise1() {
    println!("100 degree Celsius is {} Fahrenheit.", to_fahrenheit(100.0));
    println!("50 Fahrenheit is {} Celsius.", to_celsius(50.0));
}

fn exercise2() {
    println!("3rd Fibonacci number is {}", fibonacci_number_at(3));
    println!("15th Fibonacci number is {}", fibonacci_number_at(15));
}

fn exercise3() {
    let days: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh",
        "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    for day in days {
        println!("On the {day} of Christmas, blah blah blah....");
    }
}

fn fibonacci_number_at(mut n :i32) -> i32 {
    let mut i :i32 = 0;
    let mut j :i32 = 1;
    while n > 0 {
        let t :i32 = i + j;
        i = j;
        j = t;
        n -= 1;
    }
    j
}

fn to_celsius(t :f64) -> f64 {
    ((t - 32f64) * 5f64) / 9f64
}

fn to_fahrenheit(t :f64) -> f64 {
    (9f64 * t) / 5f64 + 32f64
}