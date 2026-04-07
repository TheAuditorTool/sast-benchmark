import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify

parser_bp = Blueprint("parser", __name__)

# vuln-code-snippet start ChainScenario0162A
@parser_bp.route("/api/parse", methods=["POST"])
def parse_document():
    xml_data = request.get_data(as_text=True)
    root = ET.fromstring(xml_data)  # vuln-code-snippet target-line ChainScenario0162A
    title = root.findtext("title") or ""
    body = root.findtext("body") or ""
    return jsonify({"title": title, "body": body})
# vuln-code-snippet end ChainScenario0162A
