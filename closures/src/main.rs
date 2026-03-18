use std::thread;
use std::time::Duration;

fn simulated_calculation(intensity: u32) -> u32 {
    println!("Simulating slow calculation..");
    thread::sleep(Duration::from_secs(2));
    intensity + 1
}


fn main() {
    let simulated_intensity = 10;
    let simulated_rnd_n = 7;


}

fn generated_workoout(inte: u32, r_n: u32) {
    let x_cl = |intensity: u32| -> u32 {
        simulated_calculation(intensity)
    };
    if inte < 25 {
        println!("Do {} pushps", x_cl(inte));
        println!("Next, do {} situps", x_cl(inte));
    } else {
        if r_n == 3 {
            println!("Take a break");
        } else {
            println!("Run {} minutes", x_cl(inte));
        }
    }
}
