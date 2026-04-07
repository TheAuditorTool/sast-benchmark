import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify, Response
import module_b

dav_bp = Blueprint("dav", __name__)

_DAV_NS = "DAV:"

def _safe_fromstring(text):
    parser = ET.XMLParser()
    parser.entity = {}
    return ET.fromstring(text, parser=parser)

@dav_bp.route("/dav/<path:resource>", methods=["PROPFIND"])
def propfind(resource):
    raw = request.get_data()
    if len(raw) > config.MAX_PROPFIND_BYTES:
        return Response("Request too large", status=413)
    if not raw:
        requested_props = []
    else:
        xml_text = raw.decode("utf-8", errors="replace")
        try:
            root = _safe_fromstring(xml_text)
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
