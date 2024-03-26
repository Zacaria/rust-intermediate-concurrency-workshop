// Calculated in 0.0799

use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

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
    let candidates: Vec<usize> = (0..MAX_NUM).collect();

    let start = Instant::now();
    let primes: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::new()));

    std::thread::scope(|scope| {
        let chunks = candidates.chunks(candidates.len() / num_cpus);

        for chunk in chunks {
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
