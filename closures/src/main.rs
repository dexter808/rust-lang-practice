use std::thread;
use std::time::Duration;

fn simulated_calculation(intensity: u32) -> u32 {
    println!("Simulating slow calculation..");
    thread::sleep(Duration::from_secs(2));
    intensity + 1
}

struct Cacher<T>
where T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher { calculation: calculation, value: None }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let simulated_intensity = 10;
    let simulated_rnd_n = 7;

    generated_workoout(simulated_intensity, simulated_rnd_n);
}

fn generated_workoout(inte: u32, r_n: u32) {
    let mut x_cl = Cacher::new(simulated_calculation);
    if inte < 25 {
        println!("Do {} pushps", x_cl.value(inte));
        println!("Next, do {} situps", x_cl.value(inte));
    } else {
        if r_n == 3 {
            println!("Take a break");
        } else {
            println!("Run {} minutes",x_cl.value(inte));
        }
    }
}
