"""RSS feed parser -- SAFE variant.

POST /feed/import reads an RSS 2.0 body and parses it with a hardened
XMLParser. Setting parser.entity = {} prevents any entity reference from
being mapped to an external URI, so malicious payloads that reference
file:///etc/passwd are silently dropped instead of resolved.

Chain broken: XMLParser with entity={} blocks external entity resolution ->
local file read is prevented
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


def _safe_fromstring(text):
    parser = ET.XMLParser()
    parser.entity = {}
    return ET.fromstring(text, parser=parser)


@feed_bp.route("/feed/import", methods=["POST"])
def import_feed():
    """Accept an RSS body and return the parsed feed items."""
    raw = request.get_data()
    if len(raw) > config.MAX_FEED_BYTES:
        return jsonify({"error": "feed too large"}), 413
    xml_text = raw.decode("utf-8", errors="replace")
    try:
# vuln-code-snippet start chain_rss_xxe_safe
        root = _safe_fromstring(xml_text)  # vuln-code-snippet safe-line chain_rss_xxe_safe
# vuln-code-snippet end chain_rss_xxe_safe
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    items = _extract_items(root)
    return jsonify({"items": items})
