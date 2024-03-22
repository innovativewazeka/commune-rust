use pyo3::prelude::*;
use std::thread;
use std::time::Instant;

fn sum_fn(start: usize, end: usize) -> usize {
    let mut sum: usize = 0;
    for i in start..=end {
        sum += i;
    }
    sum
}

fn main() -> PyResult<()> {
    // Initialize Python interpreter in free-threaded mode
    pyo3::prepare_freethreaded_python();


    let start_time = Instant::now();

    // Spawn a new thread to run the Python code
    let handle = thread::spawn(|| {
        let sum = sum_fn(1, 50000000);
    });

    // Wait for the thread to finish execution
    handle.join().unwrap();

    let elapsed_time = start_time.elapsed();
    // Print the elapsed time
    println!("Thread execution time: {:?}", elapsed_time);

    Ok(())
}
