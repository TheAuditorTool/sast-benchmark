"""SOAP request handler -- VULNERABLE variant.

POST /soap reads the raw request body and parses it as XML using
xml.etree.ElementTree.fromstring with no restrictions. An attacker
can inject an XXE payload in the SOAP envelope to read arbitrary
files from the server filesystem, e.g.:

    <?xml version="1.0"?>
    <!DOCTYPE env [<!ENTITY xxe SYSTEM "file:///etc/shadow">]>
    <env:Envelope xmlns:env="http://schemas.xmlsoap.org/soap/envelope/">
      <env:Body><op>&xxe;</op></env:Body>
    </env:Envelope>

Chain: SOAP body -> ET.fromstring (entities enabled) -> local file read
Individual findings: XXE in SOAP endpoint (high)
Chain finding: XXE enables reading /etc/shadow or other sensitive files (critical)
CWE-611: Improper Restriction of XML External Entity Reference
"""
import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify
import config

soap_bp = Blueprint("soap", __name__)

_SOAP_NS = "http://schemas.xmlsoap.org/soap/envelope/"


@soap_bp.route("/soap", methods=["POST"])
def handle_soap():
    """Parse incoming SOAP envelope and dispatch the body operation."""
    body_bytes = request.get_data()
    if len(body_bytes) > config.MAX_BODY_BYTES:
        return jsonify({"fault": "request too large"}), 413
    xml_text = body_bytes.decode("utf-8", errors="replace")
    try:
# vuln-code-snippet start chain_soap_xxe_vuln
        root = ET.fromstring(xml_text)  # vuln-code-snippet vuln-line chain_soap_xxe_vuln
# vuln-code-snippet end chain_soap_xxe_vuln
    except ET.ParseError as exc:
        return jsonify({"fault": str(exc)}), 400
    body_el = root.find(f"{{{_SOAP_NS}}}Body")
    if body_el is None:
        return jsonify({"fault": "missing Body element"}), 400
    result = {el.tag: el.text for el in body_el}
    return jsonify({"response": result})
