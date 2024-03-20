use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

type ThreadMap = Arc<Mutex<HashMap<String, thread::JoinHandle<()>>>>;

#[pyclass]
struct ThreadManager {
    thread_map: ThreadMap,
}

impl ThreadManager {
    fn new() -> Self {
        ThreadManager {
            thread_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[pymethods]
impl ThreadManager {
    #[staticmethod]
    fn thread(
        _py: Python,
        fn_name: String,
        args: Option<Vec<PyObject>>,
        kwargs: Option<HashMap<String, PyObject>>,
        daemon: Option<bool>,
        tag: Option<String>,
        start: Option<bool>,
        tag_separator: Option<String>,
    ) -> PyResult<()> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        // Convert args and kwargs to Python objects
        let py_args: Vec<PyObject> = args.unwrap_or_else(|| vec![]).iter().cloned().collect();
        let py_kwargs: HashMap<String, PyObject> = kwargs.unwrap_or_else(|| HashMap::new());

        // Resolve the function by name
        let fn_obj = py.getattr(&fn_name)?;

        // Create a closure to execute the function
        let closure = move || {
            // Call the function with args and kwargs
            fn_obj.call(py, (py_args.as_slice(), py_kwargs)).unwrap();
            Ok(())
        };

        // Spawn a new thread with the closure
        let handle = thread::spawn(closure).map_err(|e| PyErr::new::<exceptions::PyTypeError, _>(format!("Error spawning thread: {:?}", e)))?;
        let mut thread_map = ThreadManager::get_thread_manager(py)?.thread_map.lock().unwrap();
        thread_map.insert(name, handle);

        Ok(())
    }

    #[staticmethod]
    fn get_thread_manager(py: Python) -> PyResult<ThreadManager> {
        Ok(ThreadManager::new())
    }

    #[staticmethod]
    fn queue(_py: Python, maxsize: Option<i32>) -> PyResult<Py<PyAny>> {
        let py = unsafe { Python::assume_gil_acquired() };
        let queue_module = py.import("queue").unwrap();
        let queue_cls = queue_module.getattr("Queue").unwrap();
        let queue = queue_cls.call1((maxsize,))?;
        Ok(queue.into())
    }

    #[staticmethod]
    fn join_threads(_py: Python, _threads: PyObject) -> PyResult<()> {
        let _ = Python::with_gil(|py| {
            let thread_manager = ThreadManager::get_thread_manager(py)?;
            let mut thread_map = thread_manager.thread_map.lock().unwrap();
            for (_, thread_handle) in thread_map.iter_mut() {
                let _ = thread_handle.join();
            }
            Ok(())
        });
        Ok(())
    }

    #[staticmethod]
    fn threads(_py: Python) -> PyResult<Vec<String>> {
        let thread_manager = ThreadManager::get_thread_manager(_py)?;
        let thread_map = thread_manager.thread_map.lock().unwrap();
        let keys: Vec<String> = thread_map.keys().cloned().collect();
        Ok(keys)
    }

    #[staticmethod]
    fn num_threads(_py: Python) -> usize {
        let thread_manager = ThreadManager::get_thread_manager(_py).unwrap();
        let thread_map = thread_manager.thread_map.lock().unwrap();
        thread_map.len()
    }

    #[staticmethod]
    fn test(_py: Python) -> PyResult<()> {
        let thread_manager = ThreadManager::get_thread_manager(_py)?;
        let fn_name = "fn".to_string();
        thread_manager.thread(fn_name, None, None, None, Some("test".to_string()), Some(true), None).unwrap();
        thread::sleep(std::time::Duration::from_secs(2));
        println!("done");
        Ok(())
    }

    #[staticmethod]
    fn semaphore(_py: Python, n: i32) -> PyResult<Py<PyAny>> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let threading_module = py.import("threading").unwrap();
        let semaphore_cls = threading_module.getattr("Semaphore").unwrap();
        let semaphore = semaphore_cls.call1((n,))?;
        Ok(semaphore.into())
    }

    fn get_thread_map() -> ThreadMap {
        Arc::new(Mutex::new(HashMap::new()))
    }

    fn get_thread_manager(py: Python) -> PyResult<&'static PyAny> {
        let thread_manager = py.import("thread_manager").unwrap();
        Ok(thread_manager)
    }
}

#[pymodule]
fn rust_threading(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ThreadManager>()?;
    Ok(())
}
