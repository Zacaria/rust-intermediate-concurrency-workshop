// Calculated in 0.0539

// Each thread does it's work and adds result to a shared memory

// Wind up numbers calculation between threads
// So last threads are not the only one to deal with big number
// Therefore big calculation

use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

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
    let primes: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::new()));

    candidates.shuffle(&mut rand::thread_rng());

    std::thread::scope(|scope| {
        let chunks = candidates.chunks(candidates.len() / num_cpus);
        // We'll use enumerate() to get the index of each chunk
        for chunk in chunks {
            // And check the chunk sizes
            let my_primes = primes.clone();
            scope.spawn(move || {
                let local_results: Vec<usize> =
                    chunk.iter().filter(|n| is_prime(**n)).map(|n| *n).collect();
                let mut lock = my_primes.lock().unwrap();
                lock.extend(local_results);
            });
        }
    });
    let elapsed = start.elapsed();

    let lock = primes.lock().unwrap();

    println!("Found {} primes", lock.len());
    // println!("{primes:?}");
    println!("Calculated in {:.4}", elapsed.as_secs_f32());
}
