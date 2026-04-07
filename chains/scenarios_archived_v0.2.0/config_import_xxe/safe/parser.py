"""Configuration import parser -- SAFE variant.

POST /config/import accepts an XML configuration body and parses it
with a hardened XMLParser. Clearing the entity table prevents any
external entity reference from being resolved, so payloads targeting
file:///app/.env or other paths produce no file read.

Chain broken: XMLParser with entity={} blocks external entity resolution ->
.env exfiltration is prevented
CWE-611: Improper Restriction of XML External Entity Reference
"""
import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify

config_bp = Blueprint("config", __name__)


def _safe_fromstring(text):
    parser = ET.XMLParser()
    parser.entity = {}
    return ET.fromstring(text, parser=parser)


@config_bp.route("/config/import", methods=["POST"])
def import_config():
    """Parse an XML config body and return extracted key-value settings."""
    raw = request.get_data()
    if len(raw) > 64 * 1024:
        return jsonify({"error": "config too large"}), 413
    xml_text = raw.decode("utf-8", errors="replace")
    try:
# vuln-code-snippet start chain_config_import_xxe_safe
        root = _safe_fromstring(xml_text)  # vuln-code-snippet safe-line chain_config_import_xxe_safe
# vuln-code-snippet end chain_config_import_xxe_safe
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    settings = {
        el.get("key", el.tag): el.text or ""
        for el in root
    }
    return jsonify({"settings": settings})
