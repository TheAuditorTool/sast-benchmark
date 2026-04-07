"""WebDAV PROPFIND handler -- SAFE variant.

PROPFIND /dav/<path> parses the WebDAV XML body with a hardened
XMLParser. Clearing parser.entity prevents external entity declarations
from being resolved, so crafted PROPFIND bodies that reference
/etc/ssl/private/server.key produce no file read.

Chain broken: XMLParser with entity={} blocks external entity resolution ->
private key read via WebDAV PROPFIND is prevented
CWE-611: Improper Restriction of XML External Entity Reference
"""
import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify, Response
import config

dav_bp = Blueprint("dav", __name__)

_DAV_NS = "DAV:"


def _safe_fromstring(text):
    parser = ET.XMLParser()
    parser.entity = {}
    return ET.fromstring(text, parser=parser)


@dav_bp.route("/dav/<path:resource>", methods=["PROPFIND"])
def propfind(resource):
    """Handle WebDAV PROPFIND: parse requested properties and respond."""
    raw = request.get_data()
    if len(raw) > config.MAX_PROPFIND_BYTES:
        return Response("Request too large", status=413)
    if not raw:
        requested_props = []
    else:
        xml_text = raw.decode("utf-8", errors="replace")
        try:
# vuln-code-snippet start chain_dav_xxe_safe
            root = _safe_fromstring(xml_text)  # vuln-code-snippet safe-line chain_dav_xxe_safe
# vuln-code-snippet end chain_dav_xxe_safe
        except ET.ParseError as exc:
            return Response(str(exc), status=400)
        requested_props = [el.tag for el in root.iter() if el.tag != root.tag]
    response_xml = (
        '<?xml version="1.0" encoding="utf-8"?>'
        '<D:multistatus xmlns:D="DAV:">'
        f'<D:response><D:href>/dav/{resource}</D:href>'
        "<D:propstat><D:prop>"
        + "".join(f"<D:prop>{p}</D:prop>" for p in requested_props)
        + "</D:prop><D:status>HTTP/1.1 200 OK</D:status></D:propstat>"
        "</D:response></D:multistatus>"
    )
    return Response(response_xml, status=207, mimetype="application/xml")
