// Calculated in 0.3541

use std::time::Instant;

const MAX_NUM: usize = 100_000;

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

fn main() {
    let candidates: Vec<usize> = (0..MAX_NUM).collect();

    let start = Instant::now();
    let primes: Vec<usize> = candidates
        .iter()
        .filter(|&n| is_prime(*n))
        .map(|&n| n)
        .collect();
    let elapsed = start.elapsed();

    println!("Found {} primes", primes.len());
    // println!("{primes:?}");
    println!("Calculated in {:.4}", elapsed.as_secs_f32());
}
