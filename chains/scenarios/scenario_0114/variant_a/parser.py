import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify
import config

feed_bp = Blueprint("feed", __name__)

def _extract_items(root):
    channel = root.find("channel")
    if channel is None:
        return []
    return [
        {"title": item.findtext("title") or "", "link": item.findtext("link") or ""}
        for item in channel.findall("item")
    ]

@feed_bp.route("/feed/import", methods=["POST"])
def import_feed():
    raw = request.get_data()
    if len(raw) > config.MAX_FEED_BYTES:
        return jsonify({"error": "feed too large"}), 413
    xml_text = raw.decode("utf-8", errors="replace")
    try:
# vuln-code-snippet start ChainScenario0114A
        root = ET.fromstring(xml_text)  # vuln-code-snippet target-line ChainScenario0114A
# vuln-code-snippet end ChainScenario0114A
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    items = _extract_items(root)
    return jsonify({"items": items})
