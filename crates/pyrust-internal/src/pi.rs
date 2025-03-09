use rand::distr::StandardUniform;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Debug, Default)]
struct MonteCarloPiEstimate {
    points_in_circle: usize,
    points_total: usize,
}

fn get_random_points(num_samples: usize) -> Vec<(f32, f32)> {
    let mut rng = rand::rng();
    let x: Vec<f32> = (&mut rng)
        .sample_iter(StandardUniform)
        .take(num_samples)
        .collect();
    let y: Vec<f32> = (&mut rng)
        .sample_iter(StandardUniform)
        .take(num_samples)
        .collect();

    x.into_iter().zip(y).collect()
}

fn is_in_circle(point: &(f32, f32)) -> bool {
    let (x, y) = point;
    x * x + y * y <= 1.0
}

pub fn monte_carlo_pi(num_samples: usize, n_threads: Option<usize>) -> f32 {
    let n_threads = n_threads.unwrap_or_else(num_cpus::get);
    let samples_per_thread = num_samples.div_ceil(n_threads);

    println!("Using {} threads", n_threads);
    println!(
        "Using {} samples per thread, {} total samples",
        samples_per_thread, num_samples
    );

    let mut thread_handles = Vec::with_capacity(n_threads);
    let estimates = Arc::new(Mutex::new(MonteCarloPiEstimate::default()));

    let start = Instant::now();

    for thread_id in 0..n_threads {
        let estimates = estimates.clone();
        let thread = std::thread::spawn(move || {
            println!("Thread {thread_id} started");

            let points = get_random_points(samples_per_thread);
            let points_in_circle = points.iter().filter(|&p| is_in_circle(p)).count();
            let mut estimates = estimates.lock().unwrap();
            estimates.points_in_circle += points_in_circle;
            estimates.points_total += samples_per_thread;

            println!("Thread {thread_id} finished");
        });

        thread_handles.push(thread);
    }

    for thread_handle in thread_handles {
        thread_handle.join().unwrap();
    }

    println!("Estimating pi took {:?}", start.elapsed());

    let estimates = estimates.lock().unwrap();
    4.0 * estimates.points_in_circle as f32 / estimates.points_total as f32
}
