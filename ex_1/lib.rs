use pyo3::prelude::*;

/// Take a name and say hello
#[pyfunction]
#[pyo3(signature = (name, conf="the conference".to_string()))]
fn say_hello(name: String, conf: String) -> PyResult<String> {
    Ok(format!("Hello {}, welcome to {}", name, conf))
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_101(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
