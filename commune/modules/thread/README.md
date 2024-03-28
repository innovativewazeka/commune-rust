<div align="center">

# Rust Thread Using PyO3

</div>

## PyO3

PyO3 is Rust bindings for Python, including tools for creating native Python extension modules.<br>
Running and interacting with Python code from a Rust binary is also supported.<br>

PyO3 can be used to generate a native Python module. <br>
The easiest way to try this out for the first time is to use `maturin`.<br>
`maturin` is a tool for building and publishing Rust-based Python packages with minimal configuration.

## Install

### Install Virtualenvs

If you haven't already installed virtualenvs, you can do so using python3:

```bash
python3 -m venv .venv
source .venv/bin/activate
```

### Install Maturin

Maturin builds and publishs crates with pyo3, rust-cpython and cffi bindings as well as rust binaries as python packages.
You can install it with pip:

```bash
pip install maturin
```

### Running Rust Thread

In python code, you have to import rust thread executor.

```python
import rust_thread_executor
```

And then you have to call method `create_thread` with callable object(function or method) and args as parameters.

```python
rust_thread_executor.create_thread(python_callable_object_name, args)
```

To be able to run python code, you need to compile the Rust code and install it as a Python library.

```bash
maturin develop
python3 python_file_name.py
```
