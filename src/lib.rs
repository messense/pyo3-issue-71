#![feature(proc_macro, specialization, const_fn)]
extern crate pyo3;

use pyo3::prelude::*;

#[py::modinit(_word_count)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "test")]
    fn test(py: Python, data: Vec<Vec<String>>) -> PyResult<i32> {
        Ok(1)
    }

    Ok(())
}
