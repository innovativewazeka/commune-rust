use futures::future::Future;
use futures::sync::oneshot::{channel, Sender};
use futures_cpupool::{Builder, CpuPool};
use tokio_core::reactor::Timeout;
use tokio_core::reactor::{Core, Handle, Remote};

use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};
use std::time::{Instant, Duration};

#[derive(Clone)]
pub struct TaskHandle {
    should_stop: Arc<AtomicBool>,
}

impl TaskHandle {
    fn new() -> TaskHandle {
        TaskHandle { should_stop: Arc::new(AtomicBool::new(false)) }
    }

    // Stops the correspondent task. Note that a running task won't be interrupted, but
    // future tasks executions will be prevented.
    pub fn stop(&self) {
        self.should_stop.store(true, Ordering::Relaxed);
    }

    /// Returns true if the task is stopped.
    pub fn stopped(&self) -> bool {
        self.should_stop.load(Ordering::Relaxed)
    }
}

fn fixed_interval_loop<F>(scheduled_fn: F, interval: Duration, handle: &Handle, task_handle: TaskHandle)
    where F: Fn(&Handle) + Send + 'static
{
    if task_handle.stopped() {
        return;
    }
    let start_time = Instant::now();
    scheduled_fn(handle);
    let execution = start_time.elapsed();
    let next_iter_wait = if execution >= interval {
        Duration::from_secs(0)
    } else {
        interval - execution
    };
    let handle_clone = handle.clone();
    let t = Timeout::new(next_iter_wait, handle).unwrap()
        .then(move |_| {
            fixed_interval_loop(scheduled_fn, interval, &handle_clone, task_handle);
            Ok::<(), ()>(())
        });
    handle.spawn(t);
}

fn calculate_delay(interval: Duration, execution: Duration, delay: Duration) -> (Duration, Duration) {
    if execution >= interval {
        (Duration::from_secs(0), delay + execution - interval)
    } else {
        let wait_gap = interval - execution;
        if delay == Duration::from_secs(0) {
            (wait_gap, Duration::from_secs(0))
        } else if delay < wait_gap {
            (wait_gap - delay, Duration::from_secs(0))
        } else {
            (Duration::from_secs(0), delay - wait_gap)
        }
    }
}