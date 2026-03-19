fn main() {
    let v1 = vec![1,2,3];

    let v1_iter = v1.iter(); // Iterators are lazy

    for val in v1_iter {
        println!("{}", val);
    }
    for val in &v1 {
        for val2 in &v1 {
            println!("{} - {}", val, val2);
        }
    }
}
