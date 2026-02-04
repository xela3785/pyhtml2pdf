use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use pyo3::types::PyDict;
use headless_chrome::types::PrintToPdfOptions;

/// Options for PDF generation.
///
/// This struct allows you to configure various aspects of the PDF output,
/// such as page size, margins, and orientation.
#[derive(Clone, Default, Serialize, Deserialize)]
#[pyclass(get_all, set_all)]
pub struct PdfOptions {
    /// Page size (e.g., "A4", "Letter", "Legal", "Tabloid", "A3", "A5").
    /// Default is "A4".
    #[pyo3()]
    pub page_size: Option<String>,

    /// Page orientation ("Portrait" or "Landscape").
    /// Default is "Portrait".
    #[pyo3()]
    pub page_orientation: Option<String>,

    /// Top margin (e.g., "1cm", "0.5in").
    #[pyo3()]
    pub margin_top: Option<String>,

    /// Right margin (e.g., "1cm", "0.5in").
    #[pyo3()]
    pub margin_right: Option<String>,

    /// Bottom margin (e.g., "1cm", "0.5in").
    #[pyo3()]
    pub margin_bottom: Option<String>,

    /// Left margin (e.g., "1cm", "0.5in").
    #[pyo3()]
    pub margin_left: Option<String>,

    /// HTML content for the header.
    #[pyo3()]
    pub header_html: Option<String>,

    /// HTML content for the footer.
    #[pyo3()]
    pub footer_html: Option<String>,
    
    /// Scale factor for the page rendering.
    /// Default is 1.0.
    #[pyo3()]
    pub scale: Option<f64>,
    
    /// Whether to print background graphics.
    /// Default is true.
    #[pyo3()]
    pub print_background: bool,
}

#[pymethods]
impl PdfOptions {
    /// Create a new PdfOptions instance.
    ///
    /// Args:
    ///     **kwargs: Keyword arguments to set options directly.
    ///               Available keys: page_size, page_orientation, margin_top, 
    ///               margin_right, margin_bottom, margin_left, header_html, 
    ///               footer_html, scale, print_background.
    #[new]
    #[pyo3(signature = (**kwargs))]
    fn new(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Self> {
        let mut options = Self::default();
        options.print_background = true; // Default to true

        if let Some(kwargs) = kwargs {
            macro_rules! set_opt {
                ($field:ident) => {
                    if let Some(val) = kwargs.get_item(stringify!($field))? {
                        if !val.is_none() {
                            options.$field = Some(val.extract()?);
                        }
                    }
                };
                ($field:ident, bool) => {
                    if let Some(val) = kwargs.get_item(stringify!($field))? {
                        if !val.is_none() {
                            options.$field = val.extract()?;
                        }
                    }
                };
            }

            set_opt!(page_size);
            set_opt!(page_orientation);
            set_opt!(margin_top);
            set_opt!(margin_right);
            set_opt!(margin_bottom);
            set_opt!(margin_left);
            set_opt!(header_html);
            set_opt!(footer_html);
            set_opt!(scale);
            set_opt!(print_background, bool);
        }
        Ok(options)
    }
}

impl PdfOptions {
    pub fn to_chrome_options(&self) -> PrintToPdfOptions {
        let landscape = self.page_orientation.as_deref().map(|s| s.eq_ignore_ascii_case("landscape")).unwrap_or(false);
        
        // Default A4 size in inches
        let size_lower = self.page_size.as_deref().unwrap_or("A4").to_lowercase();
        let (width, height) = match size_lower.as_str() {
            "letter" => (8.5, 11.0),
            "legal" => (8.5, 14.0),
            "tabloid" => (11.0, 17.0),
            "a3" => (11.7, 16.5),
            "a5" => (5.8, 8.3),
            _ => (8.27, 11.69), // A4
        };

        PrintToPdfOptions {
            landscape: Some(landscape),
            display_header_footer: Some(self.header_html.is_some() || self.footer_html.is_some()),
            print_background: Some(self.print_background),
            scale: self.scale,
            paper_width: Some(width),
            paper_height: Some(height),
            margin_top: self.parse_dimension(&self.margin_top),
            margin_bottom: self.parse_dimension(&self.margin_bottom),
            margin_left: self.parse_dimension(&self.margin_left),
            margin_right: self.parse_dimension(&self.margin_right),
            header_template: self.header_html.clone(),
            footer_template: self.footer_html.clone(),
            ..Default::default()
        }
    }

    fn parse_dimension(&self, dim: &Option<String>) -> Option<f64> {
        let s = dim.as_ref()?.trim().to_lowercase();

        if let Some(v) = s.strip_suffix("in") {
            v.parse().ok()
        } else if let Some(v) = s.strip_suffix("mm") {
            v.parse::<f64>().ok().map(|v| v / 25.4)
        } else if let Some(v) = s.strip_suffix("cm") {
            v.parse::<f64>().ok().map(|v| v / 2.54)
        } else {
            // Assume mm if no unit provided or unknown
            s.parse::<f64>().ok().map(|v| v / 25.4)
        }
    }
}


pub fn register_options(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PdfOptions>()?;
    Ok(())
}