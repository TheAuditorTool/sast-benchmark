import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify

config_bp = Blueprint("config", __name__)

@config_bp.route("/config/import", methods=["POST"])
def import_config():
    raw = request.get_data()
    if len(raw) > 64 * 1024:
        return jsonify({"error": "config too large"}), 413
    xml_text = raw.decode("utf-8", errors="replace")
    try:
        root = ET.fromstring(xml_text)
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    settings = {
        el.get("key", el.tag): el.text or ""
        for el in root
    }
    return jsonify({"settings": settings})
