"""Views -- SAFE variant for content_type_confusion.

GET /file?name=<filename> serves the file with a fixed safe Content-Type.

This file is IDENTICAL between vuln/ and safe/ variants (only
response_builder.py changes).
"""
from flask import Blueprint, request
from response_builder import build_response

views_bp = Blueprint("views", __name__)

_FILES = {
    "report": b"Annual report data",
    "logo": b"<binary image data>",
}


# vuln-code-snippet start chain_content_type_sniff_safe
@views_bp.route("/file")
def serve_file():
    """Serve a file with a fixed safe Content-Type."""
    name = request.args.get("name", "report")
    body = _FILES.get(name, b"Not found")
    return build_response(name, body)  # vuln-code-snippet safe-line chain_content_type_sniff_safe
# vuln-code-snippet end chain_content_type_sniff_safe
