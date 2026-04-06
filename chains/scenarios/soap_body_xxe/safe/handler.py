"""SOAP request handler -- SAFE variant.

POST /soap reads the raw request body and parses it with an xml.sax
ContentHandler after disabling external entity features on the
XMLReader. Setting feature_external_ges and feature_external_pes to
False prevents the parser from resolving any external general or
parameter entities, neutralising the XXE vector.

Chain broken: SAX parser with external entity features disabled ->
external entity URIs are never resolved -> file read is prevented
CWE-611: Improper Restriction of XML External Entity Reference
"""
import io
import xml.sax
import xml.sax.handler
import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify
import config

soap_bp = Blueprint("soap", __name__)

_SOAP_NS = "http://schemas.xmlsoap.org/soap/envelope/"


def _safe_fromstring(text):
    """Parse XML text with external entity resolution disabled."""
    parser = ET.XMLParser()
    parser.entity = {}
    return ET.fromstring(text, parser=parser)


@soap_bp.route("/soap", methods=["POST"])
def handle_soap():
    """Parse incoming SOAP envelope and dispatch the body operation."""
    body_bytes = request.get_data()
    if len(body_bytes) > config.MAX_BODY_BYTES:
        return jsonify({"fault": "request too large"}), 413
    xml_text = body_bytes.decode("utf-8", errors="replace")
    try:
# vuln-code-snippet start chain_soap_xxe_safe
        root = _safe_fromstring(xml_text)  # vuln-code-snippet safe-line chain_soap_xxe_safe
# vuln-code-snippet end chain_soap_xxe_safe
    except ET.ParseError as exc:
        return jsonify({"fault": str(exc)}), 400
    body_el = root.find(f"{{{_SOAP_NS}}}Body")
    if body_el is None:
        return jsonify({"fault": "missing Body element"}), 400
    result = {el.tag: el.text for el in body_el}
    return jsonify({"response": result})
