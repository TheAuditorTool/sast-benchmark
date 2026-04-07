"""Views -- VULNERABLE variant for content_type_confusion.

GET /file?name=<filename> serves the file with Content-Type inferred
from the caller-supplied filename extension.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Blueprint, request
from response_builder import build_response

views_bp = Blueprint("views", __name__)

_FILES = {
    "report": b"Annual report data",
    "logo": b"<binary image data>",
}


# vuln-code-snippet start chain_content_type_sniff_vuln
@views_bp.route("/file")
def serve_file():
    """Serve a file with Content-Type derived from the filename param."""
    name = request.args.get("name", "report")
    body = _FILES.get(name, b"Not found")
    return build_response(name, body)  # vuln-code-snippet vuln-line chain_content_type_sniff_vuln
# vuln-code-snippet end chain_content_type_sniff_vuln
