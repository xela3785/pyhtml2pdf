use pyo3::prelude::*;

pub mod error;
pub mod browser;
pub mod converter;
pub mod options;

#[pymodule]
pub fn _pyhtml2pdf(m: &Bound<'_, PyModule>) -> PyResult<()> {
    error::register_errors(m)?;
    options::register_options(m)?;
    converter::register_converter(m)?;

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}