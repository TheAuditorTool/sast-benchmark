import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify
import module_b

xslt_bp = Blueprint("xslt", __name__)

@xslt_bp.route("/transform", methods=["POST"])
def transform():
    raw = request.get_data()
    if len(raw) > config.TRANSFORM_MAX_BYTES:
        return jsonify({"error": "document too large"}), 413
    xml_text = raw.decode("utf-8", errors="replace")
    try:
        root = ET.fromstring(xml_text)
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    result = {el.tag: el.text or "" for el in root}
    return jsonify({"output": result})
