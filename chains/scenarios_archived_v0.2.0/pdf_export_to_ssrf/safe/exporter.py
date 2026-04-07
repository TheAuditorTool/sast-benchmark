"""PDF export endpoint -- SAFE variant.

Accepts user-supplied HTML but strips all URL references (src/href attributes)
before passing the markup to the PDF renderer. The renderer therefore has no
external URLs to follow, preventing SSRF even though url_fetcher.fetch_resource()
remains in the pipeline for legitimate server-side assets.

Chain: POST /export with attacker HTML -> strip_url_references() removes all
       src/href URLs -> render_html_to_pdf() receives sanitized HTML ->
       no URLs extracted -> fetch_resource() never called with attacker input
Individual findings: none -- URL references removed before rendering
Chain finding: none -- SSRF path broken at sanitization step (CWE-918 mitigated)
"""
import re
from flask import Blueprint, request, jsonify
from url_fetcher import fetch_resource

export_bp = Blueprint("export", __name__)

_RENDER_URL_PATTERN = re.compile(r'(?:src|href)=["\']([^"\']+)["\']', re.IGNORECASE)
_STRIP_URL_ATTRS = re.compile(r'\s*(?:src|href)=["\'][^"\']*["\']', re.IGNORECASE)


def strip_url_references(html: str) -> str:
    """Remove all src and href attributes from user-supplied HTML.

    Prevents the PDF renderer from following any attacker-controlled URL.
    Legitimate embedded images and stylesheets must be inlined as data URIs
    before submission.
    """
    return _STRIP_URL_ATTRS.sub("", html)


def render_html_to_pdf(html: str) -> bytes:
    """Simulate a PDF renderer that fetches all embedded resource URLs.

    A real implementation would call weasyprint, wkhtmltopdf, or headless
    Chrome. All of those follow URLs found in the HTML unless the HTML is
    pre-sanitized. This simulation replicates that behaviour by extracting
    URLs and fetching them via fetch_resource().
    """
    fetched_sizes = {}
    for url in _RENDER_URL_PATTERN.findall(html):
        try:
            data = fetch_resource(url)
            fetched_sizes[url] = len(data)
        except Exception:
            fetched_sizes[url] = -1
    summary = ", ".join(f"{u}:{s}" for u, s in fetched_sizes.items())
    return f"%PDF-1.4 stub resources=[{summary}]".encode()


# vuln-code-snippet start chain_pdf_ssrf_safe
@export_bp.route("/export/pdf", methods=["POST"])
def export_pdf():
    """Export a report page as PDF.

    Accepts JSON with a 'html' field containing the page markup to render.
    URL references are stripped from user HTML before rendering to prevent SSRF.
    """
    body = request.get_json(silent=True)
    if not body or "html" not in body:
        return jsonify({"error": "JSON body with 'html' field required"}), 400

    user_html = body["html"]
    if not isinstance(user_html, str) or len(user_html) > 512_000:
        return jsonify({"error": "html must be a string under 512 KB"}), 400

    sanitized_html = strip_url_references(user_html)
    pdf_bytes = render_html_to_pdf(sanitized_html)  # vuln-code-snippet safe-line chain_pdf_ssrf_safe

    return jsonify({
        "status": "rendered",
        "size_bytes": len(pdf_bytes),
    }), 200
# vuln-code-snippet end chain_pdf_ssrf_safe
