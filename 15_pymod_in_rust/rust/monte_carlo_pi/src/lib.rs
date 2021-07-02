//! Silly method calculating PI using a Monte Carlo simulation.
use num_format::{Locale, ToFormattedString};
use rand::prelude::*;
use std::thread;
use std::time::Instant;

/// Argument takes number of iterations per thread
pub fn monte_carlo_pi(iterations: u32) -> (f64, String) {
    let num_cpus = num_cpus::get() as u32; // real and virtual

    let now = Instant::now();
    let mut handles = vec![];
    for _ in 0..num_cpus {
        handles.push(thread::spawn(move || {
            let mut rng = thread_rng();

            // Count number of random points inside unit circle
            // within quarter circle segment.
            let mut inside: u64 = 0;
            for _ in 0..iterations {
                let a = rng.gen::<f64>();
                let b = rng.gen::<f64>();
                let c = a.powf(2f64) + b.powf(2f64);
                if c <= 1f64 {
                    inside += 1;
                }
            }

            inside
        }));
    }

    let mut total_inside: u64 = 0;
    for h in handles {
        total_inside += h.join().unwrap();
    }

    let total_iterations = iterations as u64 * num_cpus as u64;
    let pi = total_inside as f64 / total_iterations as f64 * 4.0;
    let calculations_string = ((total_iterations as f64 / now.elapsed().as_secs_f64()) as u64)
        .to_formatted_string(&Locale::en);
    (pi, calculations_string)
}
