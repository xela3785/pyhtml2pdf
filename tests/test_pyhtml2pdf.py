import pytest
from pyhtml2pdf import html_to_pdf, html_to_pdf_batch, PdfOptions

def test_basic_conversion():
    """Test basic HTML to PDF conversion with default options."""
    html = "<h1>Hello World</h1><p>This is a test.</p>"
    options = PdfOptions()
    pdf_bytes = html_to_pdf(html, options)
    
    assert pdf_bytes is not None
    assert len(pdf_bytes) > 0
    assert pdf_bytes.startswith(b"%PDF")

def test_custom_options():
    """Test conversion with custom options."""
    html = "<h1>Custom Options</h1>"
    options = PdfOptions(
        page_size="A4",
        page_orientation="Landscape",
        margin_top="1cm",
        margin_bottom="1cm",
        print_background=False
    )
    pdf_bytes = html_to_pdf(html, options)
    
    assert pdf_bytes is not None
    assert len(pdf_bytes) > 0
    assert pdf_bytes.startswith(b"%PDF")

def test_batch_conversion():
    """Test batch conversion."""
    requests = [
        ("<h1>Page 1</h1>", PdfOptions()),
        ("<h1>Page 2</h1>", PdfOptions(page_orientation="Landscape"))
    ]
    results = html_to_pdf_batch(requests)
    
    assert len(results) == 2
    for pdf_bytes in results:
        assert pdf_bytes is not None
        assert len(pdf_bytes) > 0
        assert pdf_bytes.startswith(b"%PDF")

def test_header_footer():
    """Test conversion with header and footer."""
    html = "<p>Content</p>"
    options = PdfOptions(
        header_html='<div style="font-size: 10px;">Header</div>',
        footer_html='<div style="font-size: 10px;">Footer</div>',
        margin_top="2cm",
        margin_bottom="2cm"
    )
    pdf_bytes = html_to_pdf(html, options)
    
    assert pdf_bytes is not None
    assert len(pdf_bytes) > 0
    assert pdf_bytes.startswith(b"%PDF")

def test_invalid_html():
    """Test with potentially problematic HTML."""
    # Even invalid HTML is usually rendered by the browser, but let's try something weird
    html = "<html><body>Unclosed tags"
    options = PdfOptions()
    pdf_bytes = html_to_pdf(html, options)
    
    assert pdf_bytes is not None
    assert len(pdf_bytes) > 0
    assert pdf_bytes.startswith(b"%PDF")
