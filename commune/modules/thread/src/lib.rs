use pyo3::prelude::*;
use std::thread;

#[pyfunction]
fn execute_python_function(python_function: PyObject) -> PyResult<()> {
    Python::with_gil(|py| {
        match python_function.call0(py) {
            Ok(_) => println!("Python function execution completed"),
            Err(err) => eprintln!("Error calling Python function: {:?}", err),
        }
    });

    Ok(())
}

#[pyfunction]
fn create_thread(_py: Python, python_function: PyObject) -> PyResult<()> {
    println!("Start Thread Execution");

    let handle = thread::spawn(|| {
        execute_python_function(python_function).expect("Error calling Python function");
    });

    println!("Thread completed");
    Ok(())
}

#[pymodule]
fn rust_thread_executor(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_thread, m)?)?;
    Ok(())
}
