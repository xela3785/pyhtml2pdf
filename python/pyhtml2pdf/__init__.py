"""
pyhtml2pdf: High-performance HTML to PDF converter using Rust and Headless Chrome.

This module provides a fast and efficient way to convert HTML content to PDF files.
It uses a Rust extension to drive a headless Chrome instance for accurate rendering.

Main functions:
    html_to_pdf(html: str, options: PdfOptions) -> bytes:
        Convert a single HTML string to PDF.

    html_to_pdf_batch(requests: List[Tuple[str, PdfOptions]]) -> List[bytes]:
        Convert multiple HTML strings to PDF in parallel.

Classes:
    PdfOptions: Configuration options for PDF generation (page size, margins, etc.).
    PdfError: Base exception for errors during conversion.
    PdfGeneratorError: Exception containing command output on failure.

Example:
    >>> from pyhtml2pdf import html_to_pdf, PdfOptions
    >>> options = PdfOptions(page_size="A4", margin_top="1cm")
    >>> pdf_bytes = html_to_pdf("<h1>Hello World</h1>", options)
    >>> with open("output.pdf", "wb") as f:
    ...     f.write(pdf_bytes)
"""

from ._pyhtml2pdf import *

__doc__ = _pyhtml2pdf.__doc__
if hasattr(_pyhtml2pdf, "__all__"):
    __all__ = _pyhtml2pdf.__all__