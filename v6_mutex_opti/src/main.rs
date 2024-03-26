// Calculated in 0.0496

// Each thread does it's work and synchronously join after

use std::time::Instant;

use rand::seq::SliceRandom;

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
    let num_cpus = num_cpus::get();
    println!("Using {num_cpus} threads");
    let mut candidates: Vec<usize> = (0..MAX_NUM).collect();

    let start = Instant::now();
    let mut primes: Vec<usize> = Vec::with_capacity(10_000);
    candidates.shuffle(&mut rand::thread_rng());

    std::thread::scope(|scope| {
        let mut handles = Vec::with_capacity(num_cpus);
        let chunks = candidates.chunks(candidates.len() / num_cpus);
        // We'll use enumerate() to get the index of each chunk
        for chunk in chunks {
            // And check the chunk sizes
            let handle =
                scope.spawn(move || chunk.iter().filter(|n| is_prime(**n)).map(|n| *n).collect());
            handles.push(handle);
        }

        for handle in handles {
            let local_result: Vec<usize> = handle.join().unwrap();
            primes.extend(local_result);
        }
    });
    let elapsed = start.elapsed();

    println!("Found {} primes", primes.len());
    // println!("{primes:?}");
    println!("Calculated in {:.4}", elapsed.as_secs_f32());
}
