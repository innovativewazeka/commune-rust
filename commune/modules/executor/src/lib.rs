#[macro_use] extern crate log;
extern crate futures;
extern crate tokio_core;
extern crate futures_cpupool;

pub mod executor;
pub use executor::{CoreExecutor, ThreadPoolExecutor};