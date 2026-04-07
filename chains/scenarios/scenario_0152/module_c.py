import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify
import module_b

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
        root = ET.fromstring(xml_text)
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    items = _extract_items(root)
    return jsonify({"items": items})
