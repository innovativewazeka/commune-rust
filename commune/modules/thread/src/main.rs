use pyo3::prelude::*;
use std::thread;

fn create_thread(py: Python, fn_name: &str, args: Vec<&PyAny>, kwargs: &PyDict) -> PyResult<PyObject> {
    let threading = py.import("threading")?;
    let fn_obj = py.get(fn_name)?;

    let args_tuple = PyTuple::new(py, args.as_slice());
    let thread = threading.call1("Thread", (fn_obj, args_tuple, kwargs))?;

    Ok(thread)
}

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let fn_name = "python_function";
        let args = vec![];
        let kwargs = PyDict::new(py);
        let thread = create_thread(py, fn_name, args, &kwargs)?;
        thread.call_method0("start")?;

        Ok(())
    })
}
