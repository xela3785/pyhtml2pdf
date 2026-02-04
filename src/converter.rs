use pyo3::prelude::*;
use pyo3::types::PyBytes;
use crate::error::PdfError;
use crate::options::PdfOptions;
use crate::browser::{get_pooled_tab, recycle_tab};
use base64::{Engine as _, engine::general_purpose};
use rayon::prelude::*;

// Internal function for single PDF conversion
fn convert_single_pdf(html: &str, options: &PdfOptions) -> Result<Vec<u8>, PdfError> {
    if html.trim().is_empty() {
        return Err(PdfError::EmptyHtml);
    }

    // Encode HTML to base64
    let html_base64 = general_purpose::STANDARD.encode(html);
    let data_url = format!("data:text/html;base64,{}", html_base64);

    let tab = get_pooled_tab()?;

    // Navigate to data URI
    if let Err(e) = tab.navigate_to(&data_url) {
        // If navigation fails, the tab might be dead, Don't recycle it
        return Err(PdfError::BrowserError(format!("Navigation failed: {}", e)));
    }

    // Wait for content load
    if let Err(e) = tab.wait_for_element("body") {
        // If wait for body fails, Don't recycle it
        return Err(PdfError::BrowserError(format!("Wait for body failed: {}", e)));
    }

    // Print to PDF
    let chrome_options = options.to_chrome_options();
    let result = tab.print_to_pdf(Some(chrome_options));

    match result {
        Ok(pdf_data) => {
            recycle_tab(tab);
            Ok(pdf_data)
        }
        Err(e) => {
            Err(PdfError::BrowserError(e.to_string()))
        }
    }
}

/// Convert a single HTML string to PDF.
///
/// Args:
///     html (str): The HTML content to convert.
///     options (PdfOptions): Options for PDF generation.
///
/// Returns:
///     bytes: The generated PDF data as bytes.
///
/// Raises:
///     PdfError: If conversion fails.
#[pyfunction]
fn html_to_pdf(html: &str, options: &PdfOptions) -> PyResult<PyObject> {
    Python::with_gil(|py| {
        let pdf_data = py.allow_threads(|| {
            convert_single_pdf(html, options)
        })?;
        Ok(PyBytes::new_bound(py, &pdf_data).into())
    })
}

/// Convert a batch of HTML strings to PDF in parallel.
///
/// Args:
///     requests (List[Tuple[str, PdfOptions]]): A list of (html, options) tuples.
///
/// Returns:
///     List[bytes]: A list of generated PDF data as bytes, corresponding to the input list.
///
/// Raises:
///     PdfError: If any conversion fails.
#[pyfunction]
fn html_to_pdf_batch(requests: Vec<(String, PdfOptions)>) -> PyResult<Vec<PyObject>> {
    Python::with_gil(|py| {
        // Release GIL and process in parallel
        let results: Vec<Result<Vec<u8>, PdfError>> = py.allow_threads(|| {
            requests.into_par_iter()
                .map(|(html, options)| {
                    convert_single_pdf(&html, &options)
                })
                .collect()
        });

        let mut py_objects = Vec::with_capacity(results.len());
        for res in results {
            let pdf_data = res
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _> (e.to_string()))?;
            py_objects.push(PyBytes::new_bound(py, &pdf_data).into());
        }
        Ok(py_objects)
    })
}

pub fn register_converter(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(html_to_pdf, m)?)?;
    m.add_function(wrap_pyfunction!(html_to_pdf_batch, m)?)?;
    Ok(())
}