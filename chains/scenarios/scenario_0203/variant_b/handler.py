import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify
import config

sitemap_bp = Blueprint("sitemap", __name__)

_SM_NS = "http://www.sitemaps.org/schemas/sitemap/0.9"

def _safe_fromstring(text):
    parser = ET.XMLParser()
    parser.entity = {}
    return ET.fromstring(text, parser=parser)

@sitemap_bp.route("/sitemap/import", methods=["POST"])
def import_sitemap():
    raw = request.get_data()
    if len(raw) > config.SITEMAP_MAX_BYTES:
        return jsonify({"error": "sitemap too large"}), 413
    xml_text = raw.decode("utf-8", errors="replace")
    try:
# vuln-code-snippet start ChainScenario0203B
        root = _safe_fromstring(xml_text)  # vuln-code-snippet target-line ChainScenario0203B
# vuln-code-snippet end ChainScenario0203B
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    urls = [
        el.text or ""
        for el in root.findall(f"{{{_SM_NS}}}url/{{{_SM_NS}}}loc")
    ]
    return jsonify({"imported": len(urls), "urls": urls})
