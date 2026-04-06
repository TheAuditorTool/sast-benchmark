"""RSS feed parser -- VULNERABLE variant.

POST /feed/import reads an RSS 2.0 body and parses it with the default
xml.etree.ElementTree parser. External entities in the feed are resolved
before item extraction, so an attacker-controlled feed with:

    <!DOCTYPE rss [<!ENTITY creds SYSTEM "file:///etc/passwd">]>
    <rss><channel><item><title>&creds;</title></item></channel></rss>

causes the server to read /etc/passwd and return its contents as the
item title.

Chain: RSS body -> ET.fromstring (entities enabled) -> local file read
Individual findings: XXE via RSS import (high)
Chain finding: XXE enables reading local files via malicious RSS feed (critical)
CWE-611: Improper Restriction of XML External Entity Reference
"""
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
    """Accept an RSS body and return the parsed feed items."""
    raw = request.get_data()
    if len(raw) > config.MAX_FEED_BYTES:
        return jsonify({"error": "feed too large"}), 413
    xml_text = raw.decode("utf-8", errors="replace")
    try:
# vuln-code-snippet start chain_rss_xxe_vuln
        root = ET.fromstring(xml_text)  # vuln-code-snippet vuln-line chain_rss_xxe_vuln
# vuln-code-snippet end chain_rss_xxe_vuln
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    items = _extract_items(root)
    return jsonify({"items": items})
