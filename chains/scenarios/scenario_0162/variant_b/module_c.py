import defusedxml.ElementTree as ET
from flask import Blueprint, request, jsonify

parser_bp = Blueprint("parser", __name__)

@parser_bp.route("/api/parse", methods=["POST"])
def parse_document():
    xml_data = request.get_data(as_text=True)
    root = ET.fromstring(xml_data)
    title = root.findtext("title") or ""
    body = root.findtext("body") or ""
    return jsonify({"title": title, "body": body})
