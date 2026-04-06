"""XSLT-style transform handler -- SAFE variant.

POST /transform accepts an XML source document, parses it with a
hardened XMLParser, and returns a JSON rendering. Clearing the
parser's entity dictionary prevents any external entity from being
resolved, so attempts to read /etc/ssl/private/server.key via entity
declarations in the source document are silently blocked.

Chain broken: XMLParser with entity={} prevents external entity
resolution -> private key file read is prevented
CWE-611: Improper Restriction of XML External Entity Reference
"""
import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify
import config

xslt_bp = Blueprint("xslt", __name__)


def _safe_fromstring(text):
    parser = ET.XMLParser()
    parser.entity = {}
    return ET.fromstring(text, parser=parser)


@xslt_bp.route("/transform", methods=["POST"])
def transform():
    """Parse XML source body and return serialized element map."""
    raw = request.get_data()
    if len(raw) > config.TRANSFORM_MAX_BYTES:
        return jsonify({"error": "document too large"}), 413
    xml_text = raw.decode("utf-8", errors="replace")
    try:
# vuln-code-snippet start chain_xslt_xxe_safe
        root = _safe_fromstring(xml_text)  # vuln-code-snippet safe-line chain_xslt_xxe_safe
# vuln-code-snippet end chain_xslt_xxe_safe
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    result = {el.tag: el.text or "" for el in root}
    return jsonify({"output": result})
