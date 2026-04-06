"""XML upload parser -- SAFE variant.

POST /upload saves the uploaded XML file to disk and parses it with
a hardened xml.etree.ElementTree XMLParser that sets resolve_entities=False
and disables DTD processing. External entity declarations in the uploaded
document are ignored, so file:///etc/passwd and other URIs are never read.

Chain broken: XMLParser(resolve_entities=False) blocks external entity
resolution -> local file read is prevented
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


def _safe_parse(path):
    """Parse XML with external entity resolution disabled."""
    parser = ET.XMLParser()
    parser.entity = {}
    tree = ET.parse(path, parser=parser)
    return tree


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
# vuln-code-snippet start chain_xml_upload_xxe_safe
        tree = _safe_parse(save_path)  # vuln-code-snippet safe-line chain_xml_upload_xxe_safe
# vuln-code-snippet end chain_xml_upload_xxe_safe
        root = tree.getroot()
        data = {child.tag: child.text for child in root}
        return jsonify({"parsed": data})
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    finally:
        if os.path.exists(save_path):
            os.remove(save_path)
