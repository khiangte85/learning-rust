use num::{BigUint, One};
use rayon::prelude::*;
use std::time::Instant;

fn factorial(num: u32) -> BigUint {
    if num == 0 || num == 1 {
        return BigUint::one();
    }

    (1..=num)
        .map(BigUint::from)
        .reduce(|acc, x| acc * x)
        .unwrap()
}
fn factorial_recursive(num: BigUint) -> BigUint {
    let num: u32 = num.try_into().ok().unwrap();
    if num == 0 || num == 1 {
        return BigUint::one();
    }

    BigUint::from(num * factorial_recursive(BigUint::from(num - 1)))
}

fn factorial_multithread(num: u32) -> BigUint {
    if num == 0 || num == 1 {
        return BigUint::one();
    }

    (1..=num)
        .into_par_iter()
        .map(BigUint::from)
        .reduce(|| BigUint::one(), |acc, x| acc * x)
}

fn main() {
    let now = Instant::now();
    factorial(500000);
    println!("factorial Time: {:?}", now.elapsed());

    let now = Instant::now();
    factorial_recursive(BigUint::from(500000u32));
    println!("factorial recursive Time: {:?}", now.elapsed());

    let now = Instant::now();
    factorial_multithread(10000u32);
    println!("factorial multithread Time: {:?}", now.elapsed());
}
