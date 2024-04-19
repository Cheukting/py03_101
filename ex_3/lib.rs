use pyo3::prelude::*;
use pyo3::exceptions::PyFileNotFoundError;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

/// Take a name and say hello
#[pyfunction]
#[pyo3(signature = (name, conf="the conference".to_string()))]
fn say_hello(name: String, conf: String) -> PyResult<String> {
    Ok(format!("Hello {}, welcome to {}", name, conf))
}

/// Give a registration list and check if name is in it
#[pyfunction]
fn check_reg(filename: String, name: String) -> PyResult<String> {
    let file_result = File::open(filename);
    match file_result {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            if contents.contains(&name) {
                Ok("You are registered!".to_string())
            } else {
                Ok("Sorry you are not in our list!".to_string())
            }
        },
        Err(_) => {
            Err(PyFileNotFoundError::new_err("File not exist"))
        },
    }
}

/// Give a list of attendee and count
#[pyfunction]
fn count_att(att_list: Vec<String>) -> PyResult<usize> {
    Ok(att_list.len())
}

/// Give a dictionary of travel budgets and calculate average
#[pyfunction]
fn travel_avg(budget_dict: HashMap<String, f32>) -> PyResult<f32> {
    let mut sum: f32 = 0.0;
    let mut count: f32 = 0.0;
    for (_, budget) in budget_dict {
        sum = sum + budget;
        count = count + 1.0;
    }
    Ok(sum/count)
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
    m.add_function(wrap_pyfunction!(check_reg, m)?)?;
    m.add_function(wrap_pyfunction!(count_att, m)?)?;
    m.add_function(wrap_pyfunction!(travel_avg, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
