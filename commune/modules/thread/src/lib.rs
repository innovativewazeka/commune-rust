use pyo3::prelude::*;
use std::thread;

#[pyfunction]
fn create_thread(_py: Python, python_function: PyObject) -> PyResult<()> {
    println!("Start Thread Execution");

    Python::with_gil(|_py| -> PyResult<()> {
        python_function.call0(_py);
        Ok(())
    });


    // let handle = thread::spawn(move || {
    //     println!("start thread");

    //     // Acquire the GIL within the thread
    //     Python::with_gil(|py| -> PyResult<()> {
    //         println!("Hello World");

    //         // // Call the Python function using the provided PyObject
    //         // match python_function.call0(py) {
    //         //     Ok(_) => println!("Python function execution completed"),
    //         //     Err(err) => eprintln!("Error calling Python function: {:?}", err),
    //         // }

    //         Ok(())
    //     }).expect("Error acquiring GIL");
    // });

    // handle.join().expect("Thread panicked");

    println!("Thread completed");
    Ok(())
}

#[pymodule]
fn rust_thread_executor(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_thread, m)?)?;
    Ok(())
}
