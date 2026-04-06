"""Configuration import parser -- VULNERABLE variant.

POST /config/import accepts an XML configuration body and parses it
with xml.etree.ElementTree's default parser. External entities are
resolved before key-value extraction, so an attacker can exfiltrate
the server's .env file with:

    <!DOCTYPE config [<!ENTITY env SYSTEM "file:///app/.env">]>
    <config><setting key="debug">&env;</setting></config>

The entity content (the full .env) appears as the setting value in
the response.

Chain: config XML body -> ET.fromstring (entities enabled) -> .env file read
Individual findings: XXE via config import (high)
Chain finding: XXE enables reading .env credentials via config import endpoint (critical)
CWE-611: Improper Restriction of XML External Entity Reference
"""
import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify

config_bp = Blueprint("config", __name__)


@config_bp.route("/config/import", methods=["POST"])
def import_config():
    """Parse an XML config body and return extracted key-value settings."""
    raw = request.get_data()
    if len(raw) > 64 * 1024:
        return jsonify({"error": "config too large"}), 413
    xml_text = raw.decode("utf-8", errors="replace")
    try:
# vuln-code-snippet start chain_config_import_xxe_vuln
        root = ET.fromstring(xml_text)  # vuln-code-snippet vuln-line chain_config_import_xxe_vuln
# vuln-code-snippet end chain_config_import_xxe_vuln
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    settings = {
        el.get("key", el.tag): el.text or ""
        for el in root
    }
    return jsonify({"settings": settings})
