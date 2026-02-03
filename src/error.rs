use pyo3::exceptions::{PyException, PyIOError, PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PdfError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("PDF generation failed: {0}")]
    GenerationError(String),

    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("Browser error: {0}")]
    BrowserError(String),

    #[error("Invalid PDF option: {0}")]
    InvalidOption(String),

    #[error("HTML content is empty")]
    EmptyHtml,
}

impl From<PdfError> for PyErr {
    fn from(err: PdfError) -> PyErr {
        match err {
            PdfError::IoError(e) => PyIOError::new_err(e.to_string()),
            PdfError::BrowserError(msg) => PyRuntimeError::new_err(msg),
            PdfError::EmptyHtml => PyValueError::new_err("HTML content cannot be empty"),
            PdfError::InvalidOption(msg) => PyValueError::new_err(msg),
            _ => PyException::new_err(err.to_string()),
        }
    }
}

#[pyclass(extends=PyException)]
#[derive(Debug)]
pub struct PdfGeneratorError {
    #[pyo3(get)]
    pub command_output: String,
}

#[pymethods]
impl PdfGeneratorError {
    #[new]
    pub fn new(command_output: String) -> Self {
        Self { command_output }
    }
}

pub fn register_errors(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("PdfError", py.get_type::<PyException>())?;
    m.add("PdfGeneratorError", py.get_type::<PdfGeneratorError>())?;
    Ok(())
}