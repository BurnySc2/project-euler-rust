#[allow(dead_code)]
#[allow(non_snake_case)]
use std::time::{Instant};

fn main() {
    let instant = Instant::now();
    let squareOfSum = (1..101).fold(0, |sum: u32, x: u32| {return sum + x}).pow(2) as u32;
    let sumOfSquares = (1..101).fold(0, |sum: u32, x: u32| {return sum + x.pow(2) as u32});
//    println!("Square of sum {}, sum of squares {}", squareOfSum, sumOfSquares);
    let solution = squareOfSum - sumOfSquares;
    println!("Solution: {}\nElapsed time: {:?}", solution, instant.elapsed());
}
