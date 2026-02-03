pub mod error;
pub mod converter;
pub mod options;

use pyo3::prelude::*;

#[pymodule]
pub fn _pyhtml2pdf(_py: Python, m: &PyModule) -> PyResult<()> {
    error::register_errors(_py, m)?;
    options::register_options(_py, m)?;
    converter::register_converter(_py, m)?;

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}