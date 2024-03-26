// Calculated in 0.1729 seconds

// is_prime yields every 1000

use std::time::Instant;
use tokio::task::JoinSet;

const MAX_NUMBER: usize = 100_000;

/// Really inefficient prime number calculator
async fn is_prime(n: usize) -> (usize, bool) {
    if n <= 1 {
        (n, false)
    } else {
        for div in 2..n {
            if n % div == 0 {
                return (n, false);
            }
            if div % 5000 == 0 {
                tokio::task::yield_now().await;
            }
        }
        (n, true)
    }
}

#[tokio::main]
async fn main() {
    let mut candidates: Vec<usize> = (0..MAX_NUMBER).collect();
    let mut tasks = JoinSet::new();

    // Perform the calculation
    let start = Instant::now();

    for i in candidates.drain(..) {
        tasks.spawn(is_prime(i));
    }

    let mut result = Vec::new();
    while let Some(Ok((n, is_prime))) = tasks.join_next().await {
        if is_prime {
            result.push(n);
        }
    }
    let elapsed = start.elapsed();

    // Results
    println!("Found {} primes", result.len());
    //println!("{:?}", primes);
    println!("Calculated in {:.4} seconds", elapsed.as_secs_f32());
}
