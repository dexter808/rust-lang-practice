
struct User {
    name: String,
    email: String,
    active: bool,
    sign_in_count: i32,
}

fn main() {
    let user1 = User {
        name: String::from("H"),
        email: String::from("example@mail.com"),
        active: true,
        sign_in_count: 1,
    };

    let name :String = String::from("A");

    let user2 = User {
        // shorthand init expression usage
        name,

        // Try commenting this line to see how value is moved for heap data
        email: String::from("Kel"),

        // struct update syntax
        ..user1
    };

    println!("User instance -- name: {}, email: {}, active: {}, sign_in_count: {}",
             user1.name, user1.email, user1.active, user1.sign_in_count);

    println!("User instance -- name: {}, email: {}, active: {}, sign_in_count: {}",
             user2.name, user2.email, user2.active, user2.sign_in_count);
}
