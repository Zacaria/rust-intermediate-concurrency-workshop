// Calculated in 0.0436 seconds

use rand::prelude::SliceRandom;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::time::Instant;

const MAX_NUMBER: usize = 100_000;

/// Really inefficient prime number calculator
fn is_prime(n: usize) -> bool {
    if n <= 1 {
        false
    } else {
        for div in 2..n {
            if n % div == 0 {
                return false;
            }
        }
        true
    }
}

fn rayon() {
    let candidates: Vec<usize> = (0..MAX_NUMBER).collect();
    // candidates.shuffle(&mut rand::thread_rng());
    // Perform the calculation
    let start = Instant::now();
    let primes = candidates
        .par_iter()
        .filter(|n| is_prime(**n))
        .map(|n| *n)
        .collect::<Vec<usize>>();
    let elapsed = start.elapsed();

    // Results
    println!("Found {} primes", primes.len());
    //println!("{:?}", primes);
    println!("Calculated in {:.4} seconds", elapsed.as_secs_f32());
}
fn rayon_with_shuffle() {
    let mut candidates: Vec<usize> = (0..MAX_NUMBER).collect();
    candidates.shuffle(&mut rand::thread_rng());
    // Perform the calculation
    let start = Instant::now();
    let primes = candidates
        .par_iter()
        .filter(|n| is_prime(**n))
        .map(|n| *n)
        .collect::<Vec<usize>>();
    let elapsed = start.elapsed();

    // Results
    println!("Found {} primes", primes.len());
    //println!("{:?}", primes);
    println!("Calculated in {:.4} seconds", elapsed.as_secs_f32());
}

fn main() {
    println!("without shuffle");
    rayon();

    println!("without shuffle");
    rayon_with_shuffle();
}
