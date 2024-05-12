use pyo3::prelude::*;
use pyo3::exceptions::PyFileNotFoundError;
use pyo3::exceptions::PyValueError;
use pyo3::exceptions::PyStopIteration;
use pyo3::types::PyType;
use pyo3::types::{PyDict, PyTuple};
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::cell::RefCell;

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

/// Class for all attendees.
#[pyclass]
struct Attendee {
    #[pyo3(get)]
    reg_num: u32,
    name: String,
    #[pyo3(get)]
    speaker: bool,
}

#[pymethods]
impl Attendee {
    #[classattr]
    fn reg_num() -> u32 {
        0
    }
    #[new]
    #[classmethod]
    fn new(cls: &Bound<'_, PyType>, name: String, speaker: bool) -> PyResult<Self> {
        if name.len() == 0 {
            Err(PyValueError::new_err("Please enter a name"))
        } else {
            let cur_num: u32 = cls.getattr("reg_num")?.extract()?;
            cls.setattr("reg_num", cur_num + 1)?;
            Ok(
                Attendee{
                    reg_num: cur_num,
                    name: name,
                    speaker: speaker,
                }
            )
        }
    }
    #[getter]
    fn get_name(&self) -> PyResult<String> {
        Ok(self.name.to_uppercase())
    }
    #[setter]
    fn set_name(&mut self, name:String) -> PyResult<()> {
        if name.len() == 0 {
            Err(PyValueError::new_err("Please enter a name"))
        } else {
            self.name = name;
            Ok(())
        }
    }
}

/// Iterator class for Fibonacci numbers.
#[pyclass]
struct Fibonacci {
    curr: u32,
    next: u32,
    max: u32,
}

#[pymethods]
impl Fibonacci {
    #[new]
    #[pyo3(signature = (max=u32::MAX/2))]
    fn new(max: u32) -> PyResult<Self> {
        Ok(Fibonacci { curr: 0, next: 1, max: max})
    }
    fn __iter__(& self) -> PyResult<Self> {
        Ok(Fibonacci { curr: self.curr, next: self.next, max: self.max })
    }
    fn __next__(&mut self) -> PyResult<u32> {
        if self.next > self.max {
            Err(PyStopIteration::new_err("Reaching the end."))
        } else {
            let current = self.curr;

            self.curr = self.next;
            self.next = current + self.next;

            Ok(current)
        }
    }
}

/// Decorator class for creating logs.
#[pyclass]
struct SaveLog {
    log: RefCell<String>,
    wraps: Py<PyAny>,
}

#[pymethods]
impl SaveLog {
    #[new]
    fn __new__(wraps: Py<PyAny>) -> Self {
        SaveLog {
            log: RefCell::new(String::new()),
            wraps: wraps,
        }
    }
    #[getter]
    fn log(&self) -> String {
        self.log.borrow().clone()
    }
    #[pyo3(signature = (*args, **kwargs))]
    fn __call__(
        &self,
        py: Python<'_>,
        args: &Bound<'_, PyTuple>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Py<PyAny>> {
        let old_log = self.log.borrow().clone();
        let ret = self.wraps.call_bound(py, args, kwargs)?;
        let new_log;
        if old_log.len() > 0 {
            new_log = format!("{}\n{}",old_log,ret);
        } else {
            new_log = format!("{}",ret);
        }
        self.log.replace(new_log);
        Ok(ret)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_101(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    m.add_function(wrap_pyfunction!(check_reg, m)?)?;
    m.add_function(wrap_pyfunction!(count_att, m)?)?;
    m.add_function(wrap_pyfunction!(travel_avg, m)?)?;
    m.add_class::<Attendee>()?;
    m.add_class::<Fibonacci>()?;
    m.add_class::<SaveLog>()?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
