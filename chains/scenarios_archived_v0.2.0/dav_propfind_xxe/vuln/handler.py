"""WebDAV PROPFIND handler -- VULNERABLE variant.

PROPFIND /dav/<path> parses the WebDAV XML body with the default
xml.etree.ElementTree parser. WebDAV PROPFIND bodies identify
requested properties using XML elements. Without external entity
restrictions, an attacker can craft a body like:

    <!DOCTYPE propfind [<!ENTITY privkey SYSTEM "file:///etc/ssl/private/server.key">]>
    <D:propfind xmlns:D="DAV:">
      <D:prop><D:displayname>&privkey;</D:displayname></D:prop>
    </D:propfind>

The private key contents are embedded in the parsed displayname and
echoed back in the multistatus response.

Chain: PROPFIND body -> ET.fromstring (entities enabled) -> private key read
Individual findings: XXE in WebDAV PROPFIND (high)
Chain finding: XXE enables reading server private key via WebDAV (critical)
CWE-611: Improper Restriction of XML External Entity Reference
"""
import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify, Response
import config

dav_bp = Blueprint("dav", __name__)

_DAV_NS = "DAV:"


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
# vuln-code-snippet start chain_dav_xxe_vuln
            root = ET.fromstring(xml_text)  # vuln-code-snippet vuln-line chain_dav_xxe_vuln
# vuln-code-snippet end chain_dav_xxe_vuln
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
