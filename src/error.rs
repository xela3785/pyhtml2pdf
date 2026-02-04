use pyo3::exceptions::{PyException, PyIOError, PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PdfError {
    /// IO error occurred.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Generic PDF generation error.
    #[error("PDF generation failed: {0}")]
    GenerationError(String),

    /// UTF-8 conversion error.
    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    /// Browser automation error.
    #[error("Browser error: {0}")]
    BrowserError(String),

    /// Invalid option provided.
    #[error("Invalid PDF option: {0}")]
    InvalidOption(String),

    /// Empty HTML content provided.
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

/// Custom error class for PDF generator errors that captures command output.
#[pyclass(extends=PyException)]
#[derive(Debug)]
pub struct PdfGeneratorError {
    /// Output from the command that failed.
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

pub fn register_errors(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("PdfError", m.py().get_type_bound::<PyException>())?;
    m.add("PdfGeneratorError", m.py().get_type_bound::<PdfGeneratorError>())?;
    Ok(())
}