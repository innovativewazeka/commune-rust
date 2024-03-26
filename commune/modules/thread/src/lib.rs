use pyo3::prelude::*;

#[pyfunction]
fn create_thread(py: Python, python_function: PyObject) -> PyResult<PyObject> {
    Ok(python_function.call0(py)?.to_object(py))
}

#[pymodule]
fn rust_thread_executor(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_thread, m)?)?;
    Ok(())
}
