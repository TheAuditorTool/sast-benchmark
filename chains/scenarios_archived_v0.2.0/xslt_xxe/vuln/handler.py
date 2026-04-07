"""XSLT-style transform handler -- VULNERABLE variant.

POST /transform accepts an XML source document, parses it with the
default xml.etree.ElementTree parser, and returns a JSON rendering.
Because external entities are not disabled, an attacker can use:

    <!DOCTYPE doc [<!ENTITY key SYSTEM "file:///etc/ssl/private/server.key">]>
    <doc><field>&key;</field></doc>

The private key content is resolved into the field text and returned
in the transformation result.

Chain: XML body -> ET.fromstring (entities enabled) -> private key file read
Individual findings: XXE in XSLT processing endpoint (high)
Chain finding: XXE enables reading server private key via transform endpoint (critical)
CWE-611: Improper Restriction of XML External Entity Reference
"""
import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify
import config

xslt_bp = Blueprint("xslt", __name__)


@xslt_bp.route("/transform", methods=["POST"])
def transform():
    """Parse XML source body and return serialized element map."""
    raw = request.get_data()
    if len(raw) > config.TRANSFORM_MAX_BYTES:
        return jsonify({"error": "document too large"}), 413
    xml_text = raw.decode("utf-8", errors="replace")
    try:
# vuln-code-snippet start chain_xslt_xxe_vuln
        root = ET.fromstring(xml_text)  # vuln-code-snippet vuln-line chain_xslt_xxe_vuln
# vuln-code-snippet end chain_xslt_xxe_vuln
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    result = {el.tag: el.text or "" for el in root}
    return jsonify({"output": result})
