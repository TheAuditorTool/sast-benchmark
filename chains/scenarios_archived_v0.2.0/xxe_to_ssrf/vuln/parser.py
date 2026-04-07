"""XML document parser -- VULNERABLE variant.

POST /api/parse accepts an XML body and parses it with the default
xml.etree.ElementTree settings, which do not restrict external entity
resolution. An attacker can submit an XML document with an external
entity referencing an internal URL (e.g., http://169.254.169.254/) to
trigger a server-side request and read the response via the parsed content.

Chain: XXE resolves external entity URL -> SSRF to internal service
Individual findings: XXE (high)
Chain finding: SSRF to secrets endpoint via XXE (critical)
"""
import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify

parser_bp = Blueprint("parser", __name__)


# vuln-code-snippet start chain_xxe_ssrf_vuln
@parser_bp.route("/api/parse", methods=["POST"])
def parse_document():
    """Parse an uploaded XML document and return extracted fields."""
    xml_data = request.get_data(as_text=True)
    root = ET.fromstring(xml_data)  # vuln-code-snippet vuln-line chain_xxe_ssrf_vuln
    title = root.findtext("title") or ""
    body = root.findtext("body") or ""
    return jsonify({"title": title, "body": body})
# vuln-code-snippet end chain_xxe_ssrf_vuln
