import io
import xml.sax
import xml.sax.handler
import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify
import module_b

soap_bp = Blueprint("soap", __name__)

_SOAP_NS = "http://schemas.xmlsoap.org/soap/envelope/"

def _safe_fromstring(text):
    parser = ET.XMLParser()
    parser.entity = {}
    return ET.fromstring(text, parser=parser)

@soap_bp.route("/soap", methods=["POST"])
def handle_soap():
    body_bytes = request.get_data()
    if len(body_bytes) > config.MAX_BODY_BYTES:
        return jsonify({"fault": "request too large"}), 413
    xml_text = body_bytes.decode("utf-8", errors="replace")
    try:
        root = _safe_fromstring(xml_text)
    except ET.ParseError as exc:
        return jsonify({"fault": str(exc)}), 400
    body_el = root.find(f"{{{_SOAP_NS}}}Body")
    if body_el is None:
        return jsonify({"fault": "missing Body element"}), 400
    result = {el.tag: el.text for el in body_el}
    return jsonify({"response": result})
