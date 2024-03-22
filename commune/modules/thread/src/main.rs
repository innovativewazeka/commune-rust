use pyo3::prelude::*;
use std::thread;
use std::time::Instant;

fn main() -> PyResult<()> {
    // Initialize Python interpreter in free-threaded mode
    pyo3::prepare_freethreaded_python();


    let start_time = Instant::now();

    // Spawn a new thread to run the Python code
    let handle = thread::spawn(|| {
        Python::with_gil(|py| {
            let fun: Py<PyAny> = PyModule::from_code(
                py,
                "def fn():
                    print('start test')
                    sum_value = 0
                    for i in range(100000001):
                        sum_value += i
                    print('end test')",
                "",
                "",
            )
            .unwrap()
            .getattr("fn")
            .unwrap()
            .into();

            fun.call0(py).unwrap();
        });
    });

    // Wait for the thread to finish execution
    handle.join().unwrap();

    let elapsed_time = start_time.elapsed();
    // Print the elapsed time
    println!("Thread execution time: {:?}", elapsed_time);

    Ok(())
}
