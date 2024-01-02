use rand::Rng;
use std::time::{Duration, Instant};

fn estimate_pi(iterations: usize) -> f64 {
    let mut inside_circle = 0;
    let mut rng = rand::thread_rng();

    for _ in 0..iterations {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        if x * x + y * y <= 1.0 {
            inside_circle += 1;
        }
    }

    4.0 * (inside_circle as f64) / (iterations as f64)
}

fn main() {
    let iterations = 10000;
    let max_executions = 10;
    let mut pi_estimates = Vec::new();
    let mut execution_times = Vec::new();

    for _ in 0..max_executions {
        let start = Instant::now();
        let pi_estimate = estimate_pi(iterations);
        let duration = start.elapsed();

        pi_estimates.push(pi_estimate);
        execution_times.push(duration);

        println!("推定値: {}, 時間: {}ms", pi_estimate, duration.as_millis());
    }

    let average_pi: f64 = pi_estimates.iter().sum::<f64>() / pi_estimates.len() as f64;
    let total_duration: Duration = execution_times.iter().sum();
    let average_duration = total_duration / max_executions as u32;

    println!("\n平均円周率の推定値: {}", average_pi);
    println!("平均実行時間: {}ms", average_duration.as_millis());
}
