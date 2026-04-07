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
    raw = request.get_data()
    if len(raw) > config.TRANSFORM_MAX_BYTES:
        return jsonify({"error": "document too large"}), 413
    xml_text = raw.decode("utf-8", errors="replace")
    try:
# vuln-code-snippet start ChainScenario0001A
        root = _safe_fromstring(xml_text)  # vuln-code-snippet target-line ChainScenario0001A
# vuln-code-snippet end ChainScenario0001A
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    result = {el.tag: el.text or "" for el in root}
    return jsonify({"output": result})
