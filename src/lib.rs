use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


#[pyfunction]
fn say_hello() {
    println!("Hello from Rust!");
    println!("Don't be rusty!");
}


#[pymodule]
fn hello_rust_test(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(say_hello));
    Ok(())
}