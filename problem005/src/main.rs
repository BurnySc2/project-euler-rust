#[allow(dead_code)]
#[allow(non_snake_case)]
use std::time::{Instant};

// greatest common divisor
fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a;
}

// largest common multiple
fn lcm(a: u64, b: u64) -> u64 {
    let div = gcd(a, b);
    if div == 1 {
        return a * b;
    }
    return a * b / div;
}

fn main() {
    let instant = Instant::now();
    let solution = (2..21).fold(1, |product: u64, x: u64| {return lcm(product, x)});

//    Runs in 640-960 ns
    println!("Solution: {}\nElapsed time: {:?}", solution, instant.elapsed());
}
