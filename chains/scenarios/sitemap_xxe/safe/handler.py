"""Sitemap import handler -- SAFE variant.

POST /sitemap/import accepts a sitemap XML body and parses it with a
hardened XMLParser whose entity dictionary is cleared before parsing.
This prevents external entity declarations from being resolved, so
payloads targeting file:///app/.env or other credential paths are
ignored rather than embedded in the URL list.

Chain broken: XMLParser with entity={} prevents entity resolution ->
credential file read is prevented
CWE-611: Improper Restriction of XML External Entity Reference
"""
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
    """Parse a sitemap XML body and return the list of discovered URLs."""
    raw = request.get_data()
    if len(raw) > config.SITEMAP_MAX_BYTES:
        return jsonify({"error": "sitemap too large"}), 413
    xml_text = raw.decode("utf-8", errors="replace")
    try:
# vuln-code-snippet start chain_sitemap_xxe_safe
        root = _safe_fromstring(xml_text)  # vuln-code-snippet safe-line chain_sitemap_xxe_safe
# vuln-code-snippet end chain_sitemap_xxe_safe
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    urls = [
        el.text or ""
        for el in root.findall(f"{{{_SM_NS}}}url/{{{_SM_NS}}}loc")
    ]
    return jsonify({"imported": len(urls), "urls": urls})
