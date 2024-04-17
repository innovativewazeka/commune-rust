extern crate futures;
extern crate tokio_core;
extern crate futures_cpupool;

pub mod threadpoolexecutor;
pub use threadpoolexecutor::{CoreExecutor, ThreadPoolExecutor};