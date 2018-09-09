#[allow(dead_code)]
#[allow(non_snake_case)]
use std::time::{Instant};
use std::collections::HashSet;
use std::cmp;

fn is_palin(number: u32) -> bool {
    let string = number.to_string();
    let rev = string.chars().rev().collect::<String>();
    if string == rev {
        return true
    }
    false
}

fn main() {
    let instant = Instant::now();

    let mut greatest = 0;
    for x in (100..1000).rev() {
        let biggestFactor = x * x;
        if biggestFactor < greatest {
            break;
        }
        for y in 100..x {
            let product = x * y as u32;
            if is_palin(product) && greatest < product {
                greatest = product;
            }
        }
    }

//  Runs in 22 ms
    println!("Solution: {}\nElapsed time: {:?}", greatest, instant.elapsed());
}
