//#![feature(specialization)]

extern crate pyo3;
use pyo3::prelude::*;

mod load_or_fetch_data;

use load_or_fetch_data::{
    test_geo as lofd_test_geo,
    fetch_data as lofd_fetch_data
};




#[pyfunction]
fn adder(a: i64, b: i64) -> i64 {
    a + b
}

#[pyfunction]
fn test_geo() -> String {
    lofd_test_geo()
}

#[pyfunction]
fn fetch_data(py:Python, url:String) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        Ok(lofd_fetch_data(&url).await)
    })
}

#[pymodule]
fn megalinref(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(adder, m)?)?;
    m.add_function(wrap_pyfunction!(test_geo, m)?)?;
    m.add_function(wrap_pyfunction!(fetch_data, m)?)?;
    Ok(())
}