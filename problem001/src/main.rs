use std::time::{Instant};
use std::ops::{Range};

fn main() {
    let instant = Instant::now();
    let range: Range<u32> = 3..1000;
//    let sum = range.fold(0, |acc: u32, x: u32| {if x % 3 == 0 || x % 5 == 0 {return y + x;} return y});
    let mut sum: u32 = 0;
    for i in range {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
//         match i {
//             x if i % 3 == 0 || i % 5 == 0 => {
//                 sum += x
//             },
//             _ => {}
//         }
    }

//    Runs in 1.28 µs
    println!("Solution: {}\nElapsed time: {:?}", sum, instant.elapsed());
}
