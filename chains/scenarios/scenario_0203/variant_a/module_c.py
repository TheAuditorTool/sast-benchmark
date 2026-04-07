import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify
import module_b

sitemap_bp = Blueprint("sitemap", __name__)

_SM_NS = "http://www.sitemaps.org/schemas/sitemap/0.9"

@sitemap_bp.route("/sitemap/import", methods=["POST"])
def import_sitemap():
    raw = request.get_data()
    if len(raw) > config.SITEMAP_MAX_BYTES:
        return jsonify({"error": "sitemap too large"}), 413
    xml_text = raw.decode("utf-8", errors="replace")
    try:
        root = ET.fromstring(xml_text)
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    urls = [
        el.text or ""
        for el in root.findall(f"{{{_SM_NS}}}url/{{{_SM_NS}}}loc")
    ]
    return jsonify({"imported": len(urls), "urls": urls})
