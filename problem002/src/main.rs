#[allow(dead_code)]
use std::time::{Instant};

fn main() {
    let instant = Instant::now();

    let limit = u32::pow(10, 6) * 4;
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    let mut sum: u32 = 0;

    loop {
        let old_b = b;
        b += a;
        if b > limit {
            break;
        }
        a = old_b;
        if b % 2 == 0 {
            sum += b;
        }
    }

//    Runs in 320 ns
    println!("Solution: {}\nElapsed time: {:?}", sum, instant.elapsed());
}
