#[allow(dead_code)]
#[allow(non_snake_case)]
use std::time::{Instant};
use std::collections::HashSet;
use std::cmp;


fn is_prime(number: &u64, primesHash: &mut HashSet<u64>, primes: &mut Vec<u64>) -> bool {
    if primesHash.contains(number) {
        return true;
    }


    let upperLimit = (*number as f64).sqrt() as u64 + 1;
    for i in 2..upperLimit {
        match i {
            _x2 if number % i == 0 => {
                return false;
            },
            _ => {}
        }
    }
    primesHash.insert(*number);
    primes.push(*number);
    return true;
}

fn main() {
    let instant = Instant::now();

//    Cache already calculated primes
    let mut primesHash: HashSet<u64> = HashSet::new();
    let mut primes: Vec<u64> = vec![];

    let mut largestPrime: u64 = 0;
    let mut targetNumber: u64 = 600851475143;

    let mut currentNumber: u64 = 2;
    loop {
        if is_prime(&currentNumber, &mut primesHash, &mut primes) {
            if targetNumber % currentNumber == 0 {
                println!("Dividing targetNumber {} through currentNumber {}", targetNumber, currentNumber);
                largestPrime = cmp::max(currentNumber, largestPrime);
                targetNumber /= currentNumber;
                currentNumber = 1;
            }
        }

        if targetNumber == 1 {
//            println!("Number found!");
            break;
        }
        if targetNumber < currentNumber {
//            println!("Breaking because current number {} is bigger than targetNumber {}. Something is wrong.", currentNumber, targetNumber);
            break;
        }

        currentNumber += 1;
    }

//  Runs in 3.5 ms
    println!("Solution: {}\nElapsed time: {:?}", largestPrime, instant.elapsed());
}
