use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::thread;
use std::time::Duration;
use std::collections::hash_map::Entry;

fn simulated_calculation(intensity: u32) -> u32 {
    println!("Simulating slow calculation..");
    thread::sleep(Duration::from_secs(2));
    intensity + 1
}

struct Cacher<T, U: Copy+Display+Eq+Hash>
where T: Fn(U) -> U,
{
    calculation: T,
    value: HashMap<U,U>,
}

impl<T, U: Copy+Display+Eq+Hash> Cacher<T, U>
where
    T: Fn(U) -> U,
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher { calculation: calculation, value: HashMap::new() }
    }

    fn value(&mut self, arg: U) -> U {
        match self.value.entry(arg) {
            Entry::Occupied(v) => *v.get(),
            Entry::Vacant(vac_e) => {
                let cv = (self.calculation)(arg);
                vac_e.insert(cv);
                cv
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
    let sv = vec![1,2];
    let mut x_cl = Cacher::new(move |arg| {
        println!("Access to value outside of current scope = {:?}",sv);
        simulated_calculation(arg)
    });

    // println!(" Value of sv {:?}", sv);

    sample_ff(&mut x_cl, inte);

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

fn sample_ff(cache: &mut Cacher<impl Fn(u32) -> u32, u32>, intensity: u32) {
    println!("Sample FF -> {}", cache.value(intensity));
    println!("Sample FF -> {}", cache.value(9));
    println!("Sample FF -> {}", cache.value(8));
    println!("Sample FF -> {}", cache.value(9));
    println!("Sample FF -> {}", cache.value(intensity));
}