"""XML document parser -- SAFE variant.

POST /api/parse uses defusedxml to parse the XML body. defusedxml
disables external entity expansion, DTD processing, and related
XML attack vectors. The XXE-to-SSRF chain is broken because external
entity URLs are never fetched.

Chain broken: defusedxml blocks external entities -> no SSRF
"""
import defusedxml.ElementTree as ET
from flask import Blueprint, request, jsonify

parser_bp = Blueprint("parser", __name__)


# vuln-code-snippet start chain_xxe_ssrf_safe
@parser_bp.route("/api/parse", methods=["POST"])
def parse_document():
    """Parse an uploaded XML document and return extracted fields."""
    xml_data = request.get_data(as_text=True)
    root = ET.fromstring(xml_data)  # vuln-code-snippet safe-line chain_xxe_ssrf_safe
    title = root.findtext("title") or ""
    body = root.findtext("body") or ""
    return jsonify({"title": title, "body": body})
# vuln-code-snippet end chain_xxe_ssrf_safe
