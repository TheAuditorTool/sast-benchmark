"""Sitemap import handler -- VULNERABLE variant.

POST /sitemap/import accepts a sitemap XML body and parses it with the
default xml.etree.ElementTree parser. The Sitemap protocol (sitemaps.org)
uses namespace http://www.sitemaps.org/schemas/sitemap/0.9. An attacker
can submit a document like:

    <!DOCTYPE sitemap [<!ENTITY db SYSTEM "file:///app/.env">]>
    <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
      <url><loc>&db;</loc></url>
    </urlset>

The external entity is resolved and the file contents are returned as
one of the imported URL locations.

Chain: sitemap body -> ET.fromstring (entities enabled) -> DB credential file read
Individual findings: XXE via sitemap import (high)
Chain finding: XXE enables reading DB credential files via sitemap import (critical)
CWE-611: Improper Restriction of XML External Entity Reference
"""
import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify
import config

sitemap_bp = Blueprint("sitemap", __name__)

_SM_NS = "http://www.sitemaps.org/schemas/sitemap/0.9"


@sitemap_bp.route("/sitemap/import", methods=["POST"])
def import_sitemap():
    """Parse a sitemap XML body and return the list of discovered URLs."""
    raw = request.get_data()
    if len(raw) > config.SITEMAP_MAX_BYTES:
        return jsonify({"error": "sitemap too large"}), 413
    xml_text = raw.decode("utf-8", errors="replace")
    try:
# vuln-code-snippet start chain_sitemap_xxe_vuln
        root = ET.fromstring(xml_text)  # vuln-code-snippet vuln-line chain_sitemap_xxe_vuln
# vuln-code-snippet end chain_sitemap_xxe_vuln
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    urls = [
        el.text or ""
        for el in root.findall(f"{{{_SM_NS}}}url/{{{_SM_NS}}}loc")
    ]
    return jsonify({"imported": len(urls), "urls": urls})
