"""XML upload parser -- VULNERABLE variant.

POST /upload saves the uploaded XML file to disk and parses it with
xml.etree.ElementTree's default settings. The default parser enables
external entity resolution, so a document declaring:

    <!DOCTYPE foo [<!ENTITY xxe SYSTEM "file:///etc/passwd">]>
    <root><data>&xxe;</data></root>

causes the parser to read /etc/passwd and embed its contents in the
parsed tree, which is then returned in the JSON response.

Chain: XML file upload -> ET.parse (entities enabled) -> file:///etc/passwd read
Individual findings: XXE via file upload (high)
Chain finding: XXE enables reading arbitrary server files (critical)
CWE-611: Improper Restriction of XML External Entity Reference
"""
import os
import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify, current_app
from werkzeug.utils import secure_filename
import secrets as cfg

parser_bp = Blueprint("parser", __name__)


def _allowed(filename):
    return "." in filename and filename.rsplit(".", 1)[1].lower() in cfg.ALLOWED_EXTENSIONS


@parser_bp.route("/upload", methods=["POST"])
def upload_xml():
    """Accept an XML file upload, parse it, and return extracted data."""
    if "file" not in request.files:
        return jsonify({"error": "no file field"}), 400
    f = request.files["file"]
    if not f or not _allowed(f.filename):
        return jsonify({"error": "invalid file"}), 400
    filename = secure_filename(f.filename)
    save_path = os.path.join(current_app.config["UPLOAD_FOLDER"], filename)
    f.save(save_path)
    try:
# vuln-code-snippet start chain_xml_upload_xxe_vuln
        tree = ET.parse(save_path)  # vuln-code-snippet vuln-line chain_xml_upload_xxe_vuln
# vuln-code-snippet end chain_xml_upload_xxe_vuln
        root = tree.getroot()
        data = {child.tag: child.text for child in root}
        return jsonify({"parsed": data})
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    finally:
        if os.path.exists(save_path):
            os.remove(save_path)
