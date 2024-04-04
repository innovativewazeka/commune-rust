extern crate threadpoolexecutor;
use threadpoolexecutor::ThreadPoolExecutor;

use std::time::{Duration, Instant};
use std::thread;

fn sum_up_to_n(n: u64) -> u64 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}

fn main() {    

    let executor = ThreadPoolExecutor::new(4).expect("Thread pool creation failed");
    let start_time = Instant::now();

    executor.schedule_fixed_rate(
        Duration::from_secs(0),
        Duration::from_secs(0),
        move |_| {
            println!("> Task is being executed");
            println!("  time elapsed from start: {:?} seconds", start_time.elapsed().as_secs());
            println!("  thread: {}", thread::current().name().unwrap());
            // Emulate an expensive task
            // thread::sleep(Duration::from_secs(10));
            let result = sum_up_to_n(100000000);
            println!("{result}");
        },
    );

    println!("Task has been scheduled");
    thread::sleep(Duration::from_secs(5));
    println!("Terminating");
}