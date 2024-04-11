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

fn fixed_rate_loop<F>(scheduled_fn: F, interval: Duration, handle: &Handle, delay: Duration, task_handle: TaskHandle)
    where F: Fn(&Handle) + Send + 'static
{
    if task_handle.stopped() {
        return;
    }
    let start_time = Instant::now();
    scheduled_fn(handle);
    let execution = start_time.elapsed();
    let (next_iter_wait, updated_delay) = calculate_delay(interval, execution, delay);
    let handle_clone = handle.clone();
    let t = Timeout::new(next_iter_wait, handle).unwrap()
        .then(move |_| {
            fixed_rate_loop(scheduled_fn, interval, &handle_clone, updated_delay, task_handle);
            Ok::<(), ()>(())
        });
    handle.spawn(t);
}

struct CoreExecutorInner {
    remote: Remote,
    termination_sender: Option<Sender<()>>,
    thread_handle: Option<JoinHandle<()>>,
}

impl Drop for CoreExecutorInner {
    fn drop(&mut self) {
        let _ = self.termination_sender.take().unwrap().send(());
        let _ = self.thread_handle.take().unwrap().join();
    }
}


//  CoreExecutor
pub struct CoreExecutor {
    inner: Arc<CoreExecutorInner>
}

impl Clone for CoreExecutor {
    fn clone(&self) -> Self {
        CoreExecutor { inner: Arc::clone(&self.inner) }
    }
}

impl CoreExecutor {
    // Creates a new `CoreExecutor`.
    pub fn new() -> Result<CoreExecutor, io::Error> {
        CoreExecutor::with_name("core_executor")
    }

    // Creates a new `CoreExecutor` with the specified thread name.
    pub fn with_name(thread_name: &str) -> Result<CoreExecutor, io::Error> {
        let (termination_tx, termination_rx) = channel();
        let (core_tx, core_rx) = channel();
        let thread_handle = thread::Builder::new()
            .name(thread_name.to_owned())
            .spawn(move || {
                debug!("Core starting");
                let mut core = Core::new().expect("Failed to start core");
                let _ = core_tx.send(core.remote());
                match core.run(termination_rx) {
                    Ok(v) => debug!("Core terminated correctly {:?}", v),
                    Err(e) => debug!("Core terminated with error: {:?}", e),
                }
            })?;
        let inner = CoreExecutorInner {
            remote: core_rx.wait().expect("Failed to receive remote"),
            termination_sender: Some(termination_tx),
            thread_handle: Some(thread_handle),
        };
        let executor = CoreExecutor {
            inner: Arc::new(inner)
        };
        debug!("Executor created");
        Ok(executor)
    }

    // Schedule a function for running at fixed intervals. The executor will try to run the
    // function every `interval`, but if one execution takes longer than `interval` it will delay
    // all the subsequent calls.
    pub fn schedule_fixed_interval<F>(&self, initial: Duration, interval: Duration, scheduled_fn: F) -> TaskHandle
        where F: Fn(&Handle) + Send + 'static
    {
        let task_handle = TaskHandle::new();
        let task_handle_clone = task_handle.clone();
        self.inner.remote.spawn(move |handle| {
            let handle_clone = handle.clone();
            let t = Timeout::new(initial, handle).unwrap()
                .then(move |_| {
                    fixed_interval_loop(scheduled_fn, interval, &handle_clone, task_handle_clone);
                    Ok::<(), ()>(())
                });
            handle.spawn(t);
            Ok::<(), ()>(())
        });
        task_handle
    }

    // Schedule a function for running at fixed rate. The executor will try to run the function
    // every `interval`, and if a task execution takes longer than `interval`, the wait time
    // between task will be reduced to decrease the overall delay.
    pub fn schedule_fixed_rate<F>(&self, initial: Duration, interval: Duration, scheduled_fn: F) -> TaskHandle
        where F: Fn(&Handle) + Send + 'static
    {
        let task_handle = TaskHandle::new();
        let task_handle_clone = task_handle.clone();
        self.inner.remote.spawn(move |handle| {
            let handle_clone = handle.clone();
            let t = Timeout::new(initial, handle).unwrap()
                .then(move |_| {
                    fixed_rate_loop(scheduled_fn, interval, &handle_clone, Duration::from_secs(0), task_handle_clone);
                    Ok::<(), ()>(())
                });
            handle.spawn(t);
            Ok::<(), ()>(())
        });
        task_handle
    }
}