pub fn direct_approach() {
    let w :u32 = 5;
    let h :u32 = 10;

    println!("Area with w={}, h={} -> {}", w, h, area(w, h));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
