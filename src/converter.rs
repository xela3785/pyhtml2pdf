use pyo3::prelude::*;
use pyo3::types::PyBytes;

use crate::error::PdfError;
use crate::options::PdfOptions;
use headless_chrome::{Browser, LaunchOptions};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::sync::Arc;
use base64::{Engine as _, engine::general_purpose};

static BROWSER: Lazy<Mutex<Option<Arc<Browser>>>> = Lazy::new(|| Mutex::new(None));

fn get_browser() -> Result<Arc<Browser>, PdfError> {
    let mut browser_guard = BROWSER.lock().map_err(|_| PdfError::BrowserError("Failed to lock browser mutex".to_string()))?;

    if let Some(browser) = browser_guard.as_ref() {
        return Ok(browser.clone())
    }

    let launch_options = LaunchOptions {
        idle_browser_timeout: std::time::Duration::from_secs(300),
        ..Default::default()
    };

    let browser = Browser::new(launch_options)
        .map_err(|e| PdfError::BrowserError(e.to_string()))?;

    let browser = Arc::new(browser);
    *browser_guard = Some(browser.clone());

    Ok(browser)
}

pub fn convert_html_to_pdf(html: &str, options: &PdfOptions) -> Result<Vec<u8>, PdfError> {
    if html.trim().is_empty() {
        return Err(PdfError::EmptyHtml);
    }

    // Encode HTML to base64
    let base64_html = general_purpose::STANDARD.encode(html);
    let data_url = format!("data:text/html;base64,{}", base64_html);

    // Get browser instance
    let browser = get_browser()?;

    // Create new browser context (incognito/isolated)
    let context = browser.new_context()
        .map_err(|e| PdfError::BrowserError(format!("Failed to create browser context: {}", e)))?;

    // Open tab in the new context
    let tab = context.new_tab()
        .map_err(|e| PdfError::BrowserError(format!("Failed to create browser tab: {}", e)))?;

    // Navigate to data URL
    tab.navigate_to(&data_url)
        .map_err(|e| PdfError::BrowserError(e.to_string()))?;

    // Print to PDF
    let chrome_options = options.to_chrome_options();
    let pdf_data = tab.print_to_pdf(Some(chrome_options))
        .map_err(|e| PdfError::BrowserError(e.to_string()))?;

    Ok(pdf_data)
}

#[pyclass]
pub struct HtmlToPdfConverter {
    options: PdfOptions,
}

#[pymethods]
impl HtmlToPdfConverter {
    #[new]
    #[pyo3(signature = (options = None))]
    fn new(options: Option<PdfOptions>) -> Self {
        Self {
            options: options.unwrap_or_default(),
        }
    }

    fn convert(&self, html: &str) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            // Release GIL during heavy conversion
            let pdf_data = py.allow_threads(|| {
                convert_html_to_pdf(html, &self.options)
            })?;
            Ok(PyBytes::new(py, &pdf_data).into())
        })
    }

    #[getter]
    fn options(&self) -> PdfOptions {
        self.options.clone()
    }

    #[setter]
    fn set_options(&mut self, options: PdfOptions) {
        self.options = options;
    }

}

#[pyfunction]
fn  html_to_pdf(html: &str) -> PyResult<PyObject> {
    Python::with_gil(|py| {
        let options = PdfOptions::default();
        let pdf_data = py.allow_threads(|| {
            convert_html_to_pdf(html, &options)
        })?;
        Ok(PyBytes::new(py, &pdf_data).into())
    })
}

#[pyfunction]
fn html_to_pdf_with_options(html: &str, options: &PdfOptions) -> PyResult<PyObject> {
    Python::with_gil(|py| {
        let pdf_data = py.allow_threads(|| {
            convert_html_to_pdf(html, options)
        })?;
        Ok(PyBytes::new(py, &pdf_data).into())
    })
}

pub fn register_converter(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(html_to_pdf, m)?)?;
    m.add_function(wrap_pyfunction!(html_to_pdf_with_options, m)?)?;
    m.add_class::<HtmlToPdfConverter>()?;
    Ok(())
}