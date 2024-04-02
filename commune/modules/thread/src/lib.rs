use pyo3::prelude::*;
use std::thread;
use pyo3::types::PyTuple;

#[pyfunction]
fn execute_python_function(python_function: PyObject, args: Vec<PyObject>) -> PyResult<()> {
    println!("Python function execution started");
    Python::with_gil(|py| {
        let args_tuple = PyTuple::new(py, args);

        match python_function.call1(py, args_tuple) {
            Ok(_) => println!("Python function execution completed"),
            Err(err) => eprintln!("Error calling Python function: {:?}", err),
        }
    });

    Ok(())
}

#[pyfunction]
fn create_thread(_py: Python, python_function: PyObject, args: Vec<PyObject>) -> PyResult<()> {
    println!("Start Thread Execution");

    let _handle = thread::spawn(|| {
        execute_python_function(python_function, args).expect("Error calling Python function");
    });

    println!("Thread completed");
    Ok(())
}

#[pyfunction]
fn execute_python_function_allow_threads(py: Python<'_>, python_function: PyObject, args: Vec<PyObject>) -> PyResult<()> {
    py.allow_threads(|| execute_python_function(python_function, args))
}

#[pymodule]
fn rust_thread_executor(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_thread, m)?)?;
    m.add_function(wrap_pyfunction!(execute_python_function_allow_threads, m)?)?;
    Ok(())
}
