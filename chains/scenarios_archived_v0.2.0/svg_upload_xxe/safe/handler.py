"""SVG upload handler -- SAFE variant.

POST /avatar saves the uploaded SVG and parses it using a hardened
XMLParser with an empty entity dictionary. This prevents the parser
from mapping any entity reference to an external URI, so
file:///etc/passwd and similar payloads are silently ignored rather
than resolved.

Chain broken: XMLParser with entity={} prevents external entity
resolution -> file contents are never embedded in the response
CWE-611: Improper Restriction of XML External Entity Reference
"""
import os
import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify, current_app
from werkzeug.utils import secure_filename
import config

svg_bp = Blueprint("svg", __name__)

_SVG_NS = "http://www.w3.org/2000/svg"


def _allowed(filename):
    return "." in filename and filename.rsplit(".", 1)[1].lower() in config.ALLOWED_EXTENSIONS


def _safe_parse(path):
    parser = ET.XMLParser()
    parser.entity = {}
    return ET.parse(path, parser=parser)


@svg_bp.route("/avatar", methods=["POST"])
def upload_avatar():
    """Save an SVG avatar and return extracted title/description metadata."""
    if "file" not in request.files:
        return jsonify({"error": "no file field"}), 400
    f = request.files["file"]
    if not f or not _allowed(f.filename):
        return jsonify({"error": "only SVG files accepted"}), 400
    filename = secure_filename(f.filename)
    save_path = os.path.join(current_app.config["UPLOAD_FOLDER"], filename)
    f.save(save_path)
    try:
# vuln-code-snippet start chain_svg_xxe_safe
        tree = _safe_parse(save_path)  # vuln-code-snippet safe-line chain_svg_xxe_safe
# vuln-code-snippet end chain_svg_xxe_safe
        root = tree.getroot()
        title = root.findtext(f"{{{_SVG_NS}}}title") or root.findtext("title") or ""
        desc = root.findtext(f"{{{_SVG_NS}}}desc") or root.findtext("desc") or ""
        return jsonify({"title": title, "description": desc})
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    finally:
        if os.path.exists(save_path):
            os.remove(save_path)
