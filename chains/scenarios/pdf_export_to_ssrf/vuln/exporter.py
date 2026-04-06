"""PDF export endpoint -- VULNERABLE variant.

Accepts user-supplied HTML for a report page and passes it directly to the
PDF rendering pipeline. The renderer (simulated here) follows every URL
reference found in the HTML -- img src, link href, etc. -- by calling
url_fetcher.fetch_resource(). Because user HTML is not sanitized, an attacker
can embed src="http://169.254.169.254/latest/meta-data/" or any internal
address and force the server to issue a blind SSRF request on their behalf.

Chain: POST /export with attacker HTML -> render_html_to_pdf() forwards HTML
       unchanged -> renderer extracts embedded URLs -> fetch_resource() is
       called with attacker-controlled destination -> blind SSRF
Individual findings: unsanitized user input reaches URL-fetching renderer (high)
Chain finding: blind SSRF against internal network / cloud metadata (critical,
               CWE-918)
"""
import re
from flask import Blueprint, request, jsonify
from url_fetcher import fetch_resource

export_bp = Blueprint("export", __name__)

_RENDER_URL_PATTERN = re.compile(r'(?:src|href)=["\']([^"\']+)["\']', re.IGNORECASE)


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
    # Return a stub PDF body; in production this would be real PDF bytes.
    summary = ", ".join(f"{u}:{s}" for u, s in fetched_sizes.items())
    return f"%PDF-1.4 stub resources=[{summary}]".encode()


# vuln-code-snippet start chain_pdf_ssrf_vuln
@export_bp.route("/export/pdf", methods=["POST"])
def export_pdf():
    """Export a report page as PDF.

    Accepts JSON with a 'html' field containing the page markup to render.
    """
    body = request.get_json(silent=True)
    if not body or "html" not in body:
        return jsonify({"error": "JSON body with 'html' field required"}), 400

    user_html = body["html"]
    if not isinstance(user_html, str) or len(user_html) > 512_000:
        return jsonify({"error": "html must be a string under 512 KB"}), 400

    pdf_bytes = render_html_to_pdf(user_html)  # vuln-code-snippet vuln-line chain_pdf_ssrf_vuln

    return jsonify({
        "status": "rendered",
        "size_bytes": len(pdf_bytes),
    }), 200
# vuln-code-snippet end chain_pdf_ssrf_vuln
