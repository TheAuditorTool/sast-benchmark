import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify
import config

soap_bp = Blueprint("soap", __name__)

_SOAP_NS = "http://schemas.xmlsoap.org/soap/envelope/"

@soap_bp.route("/soap", methods=["POST"])
def handle_soap():
    body_bytes = request.get_data()
    if len(body_bytes) > config.MAX_BODY_BYTES:
        return jsonify({"fault": "request too large"}), 413
    xml_text = body_bytes.decode("utf-8", errors="replace")
    try:
# vuln-code-snippet start ChainScenario0069A
        root = ET.fromstring(xml_text)  # vuln-code-snippet target-line ChainScenario0069A
# vuln-code-snippet end ChainScenario0069A
    except ET.ParseError as exc:
        return jsonify({"fault": str(exc)}), 400
    body_el = root.find(f"{{{_SOAP_NS}}}Body")
    if body_el is None:
        return jsonify({"fault": "missing Body element"}), 400
    result = {el.tag: el.text for el in body_el}
    return jsonify({"response": result})
